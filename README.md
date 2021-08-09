This no-std crate provides the `Final`, struct which guarantees the interior
immutability of the value that it wraps. This is useful for
preserving invariants on the fields of structures, whose 'safe'
mutation would cause undefined behavior.
