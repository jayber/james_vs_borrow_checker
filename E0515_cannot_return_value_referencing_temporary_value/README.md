```
error[E0515]: cannot return value referencing temporary value
--> second\src\main.rs:43:9
|
41 |             new_entries.push(&AnEntry::new_summed(summed));
|                               --------------------------- temporary value created here
42 |         }
43 |         new_entries.into_iter()
|         ^^^^^^^^^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function
|
= help: use `.collect()` to allocate the iterator
```

Versions of this error are always popping up. AFAIK they require design change in order not to 
make a value an orphan (one that is not owned).

In this example, I am trying to return an iterator of references to a Vec of owned values (which 
is fine) by enhancing the list on the fly with new `Summed` values. The trouble comes from not 
keeping these new instances anywhere in the struct.

The `&AnEntry::new_summed(summed)` creates a new value (`AnEntry::new_summed(summed)`), which is 
owned by this function and therefore will be dropped when it finishes, meaning the 
reference would point to nothing.

The solution provided is to populate the original owned `Vec<AnEntry>` with the 
`AnEntry::Summed` values when it is populated in the first place, rather than creating them on the 
fly when an iterator is required.

I'd certainly be interested in other solutions that don't require this.