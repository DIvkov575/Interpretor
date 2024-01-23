use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::{ScopedPtr, TaggedCellPtr, TaggedScopedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::evalrus::Value::Value;
use crate::frontend::Array::{ArraySize, default_array_growth};
use crate::frontend::RawArray::RawArray;
use crate::frontend::Traits::{Container, HashIndexedAnyContainer};
use crate::internals::Errors::{ErrorKind, RuntimeError};

const LOAD_FACTOR: f32 = 0.80;
const TOMBSTONE: u64 = 1;


#[derive(Clone)]
pub struct DictItem {
    key: TaggedCellPtr,
    value: TaggedCellPtr,
    hash: u64,
}
pub struct Dict {
    /// Number of items stored
    length: Cell<ArraySize>,
    /// Total count of items plus tombstones
    used_entries: Cell<ArraySize>,
    /// Backing array for key/value entries
    data: Cell<RawArray<DictItem>>,
}

impl Dict {
    /// Allocate a new instance on the heap
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, Dict>, RuntimeError> {
        mem.alloc(Dict::new())
    }

    /// Allocate a new instance on the heap with pre-allocated capacity
    pub fn alloc_with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize,
    ) -> Result<ScopedPtr<'guard, Dict>, RuntimeError> {
        mem.alloc(Dict::with_capacity(mem, capacity)?)
    }

    /// Scale capacity up if needed
    fn grow_capacity<'guard>(&self, mem: &'guard MutatorView) -> Result<(), RuntimeError> {
        let data = self.data.get();

        let new_capacity = default_array_growth(data.capacity())?;
        let new_data = RawArray::<DictItem>::with_capacity(mem, new_capacity)?;

        let maybe_ptr = data.as_ptr();
        if let Some(ptr) = maybe_ptr {
            for index in 0..data.capacity() {
                let entry =
                    unsafe { &mut *(ptr.offset(index as isize) as *mut DictItem) as &mut DictItem };
                if !entry.key.is_nil() {
                    let new_entry = find_entry(mem, &new_data, entry.hash)?;
                    *new_entry = entry.clone();
                }
            }
        }

        self.data.set(new_data);
        Ok(())
    }
}

impl Container<DictItem> for Dict {
    fn new() -> Dict {
        Dict {
            length: Cell::new(0),
            used_entries: Cell::new(0),
            data: Cell::new(RawArray::new()),
        }
    }

    fn with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize,
    ) -> Result<Self, RuntimeError> {
        let dict = Dict {
            length: Cell::new(0),
            used_entries: Cell::new(0),
            data: Cell::new(RawArray::with_capacity(mem, capacity)?),
        };

        let data = dict.data.get();
        fill_with_blank_entries(mem, &data)?;

        Ok(dict)
    }

    fn clear<'guard>(&self, mem: &'guard MutatorView) -> Result<(), RuntimeError> {
        let data = self.data.get();
        fill_with_blank_entries(mem, &data)?;
        self.length.set(0);
        self.used_entries.set(0);
        Ok(())
    }

    fn length(&self) -> ArraySize {
        self.length.get()
    }
}

/// Hashable-indexed interface. Objects used as keys must implement Hashable.
impl HashIndexedAnyContainer for Dict {
    fn lookup<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        key: TaggedScopedPtr,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError> {
        let hash = hash_key(guard, key)?;
        let data = self.data.get();
        let entry = find_entry(guard, &data, hash)?;

        if !entry.key.is_nil() {
            Ok(entry.value.get(guard))
        } else {
            Err(RuntimeError::new(ErrorKind::KeyError))
        }
    }

    // ANCHOR: DefHashIndexedAnyContainerForDictAssoc
    fn assoc<'guard>(
        &self,
        mem: &'guard MutatorView,
        key: TaggedScopedPtr<'guard>,
        value: TaggedScopedPtr<'guard>,
    ) -> Result<(), RuntimeError> {
        let hash = hash_key(mem, key)?;

        let mut data = self.data.get();
        // check the load factor (what percentage of the capacity is or has been used)
        if needs_to_grow(self.used_entries.get() + 1, data.capacity()) {
            // create a new, larger, backing array, and copy all existing entries over
            self.grow_capacity(mem)?;
            data = self.data.get();
        }

        // find the slot whose entry matches the hash or is the nearest available entry
        let entry = find_entry(mem, &data, hash)?;

        // update counters if necessary
        if entry.key.is_nil() {
            // if `key` is nil, this entry is unused: increment the length
            self.length.set(self.length.get() + 1);
            if entry.hash == 0 {
                // if `hash` is 0, this entry has _never_ been used: increment the count
                // of used entries
                self.used_entries.set(self.used_entries.get() + 1);
            }
        }

        // finally, write the key, value and hash to the entry
        entry.key.set(key);
        entry.value.set(value);
        entry.hash = hash;

        Ok(())
    }
    // ANCHOR_END: DefHashIndexedAnyContainerForDictAssoc

    // ANCHOR: DefHashIndexedAnyContainerForDictDissoc
    fn dissoc<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        key: TaggedScopedPtr,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError> {
        let hash = hash_key(guard, key)?;

        let data = self.data.get();
        let entry = find_entry(guard, &data, hash)?;

        if entry.key.is_nil() {
            // a nil key means the key was not found in the Dict
            return Err(RuntimeError::new(ErrorKind::KeyError));
        }

        // decrement the length but not the `used_entries` count
        self.length.set(self.length.get() - 1);

        // write the "tombstone" markers to the entry
        entry.key.set_to_nil();
        entry.hash = TOMBSTONE;

        // return the value that was associated with the key
        Ok(entry.value.get(guard))
    }
    // ANCHOR_END: DefHashIndexedAnyContainerForDictDissoc

    fn exists<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        key: TaggedScopedPtr,
    ) -> Result<bool, RuntimeError> {
        let hash = hash_key(guard, key)?;
        let data = self.data.get();
        let entry = find_entry(guard, &data, hash)?;
        Ok(!entry.key.is_nil())
    }
}


fn find_entry<'guard>(
    _guard: &'guard dyn MutatorScope,
    data: &RawArray<DictItem>,
    hash: u64,
) -> Result<&'guard mut DictItem, RuntimeError> {
    // get raw pointer to base of array
    let ptr = data
        .as_ptr()
        .ok_or(RuntimeError::new(ErrorKind::BoundsError))?;

    // calculate the starting index into `data` to begin scanning at
    let mut index = (hash % data.capacity() as u64) as ArraySize;

    // the first tombstone we find will be saved here
    let mut tombstone: Option<&mut DictItem> = None;

    loop {
        let entry = unsafe { &mut *(ptr.offset(index as isize) as *mut DictItem) as &mut DictItem };

        if entry.hash == TOMBSTONE && entry.key.is_nil() {
            // this is a tombstone: save the first tombstone reference we find
            if tombstone.is_none() {
                tombstone = Some(entry);
            }
        } else if entry.hash == hash {
            // this is an exact match slot
            return Ok(entry);
        } else if entry.key.is_nil() {
            // this is a non-tombstone empty slot
            if let Some(earlier_entry) = tombstone {
                // if we recorded a tombstone, return _that_ slot to be reused
                return Ok(earlier_entry);
            } else {
                return Ok(entry);
            }
        }

        // increment the index, wrapping back to 0 when we get to the end of the array
        index = (index + 1) % data.capacity();
    }
}
// ANCHOR_END: DefFindEntry

/// Reset all slots to a blank entry
fn fill_with_blank_entries<'guard>(
    _guard: &'guard dyn MutatorScope,
    data: &RawArray<DictItem>,
) -> Result<(), RuntimeError> {
    let ptr = data
        .as_ptr()
        .ok_or(RuntimeError::new(ErrorKind::BoundsError))?;

    let blank_entry = DictItem::blank();

    for index in 0..data.capacity() {
        let entry = unsafe { &mut *(ptr.offset(index as isize) as *mut DictItem) as &mut DictItem };
        *entry = blank_entry.clone();
    }

    Ok(())
}

/// Returns true if the dict has reached it's defined load factor and needs to be resized before inserting
/// a new entry.
fn needs_to_grow(used_entries: ArraySize, capacity: ArraySize) -> bool {
    let ratio = (used_entries as f32) / (capacity as f32);
    ratio > LOAD_FACTOR
}

fn hash_key<'guard>(
    guard: &'guard dyn MutatorScope,
    key: TaggedScopedPtr<'guard>,
) -> Result<u64, RuntimeError> {
    match *key {
        Value::Symbol(s) => {
            // let mut hasher = FnvHasher::default();
            let mut hasher = DefaultHasher;
            s.hash(guard, &mut hasher);
            Ok(hasher.finish())
        }
        Value::Number(n) => Ok(n as u64),
        _ => Err(RuntimeError::new(ErrorKind::UnhashableError)),
    }
}
