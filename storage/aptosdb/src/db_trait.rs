use aptos_schemadb::{iterator::ScanDirection, ColumnFamilyName, ReadOptions, SchemaBatch};
use std::path::Path;

pub trait LDB: Send + Sync {
    type Success;
    type Error: Error + Debug;

    /// Opens database with defaults
    fn open(
        path: impl AsRef<Path>,
        name: &str,
        column_families: Vec<ColumnFamilyName>,
        db_opts: &Options,
    ) -> Result<Self, Self::Error>;

    /// opens database in cf mode
    fn open_cf(
        db_opts: &Options,
        path: impl AsRef<Path>,
        name: &str,
        cfds: Vec<ColumnFamilyDescriptor>,
    ) -> Result<Self, Self::Error>;

    /// Open db in readonly mode
    /// Note that this still assumes there's only one process that opens the same DB.
    /// See `open_as_secondary`
    fn open_cf_readonly(
        opts: &Options,
        path: impl AsRef<Path>,
        name: &str,
        cfs: Vec<ColumnFamilyName>,
    ) -> Result<Self, Self::Error>;

    fn open_cf_as_secondary<P: AsRef<Path>>(
        opts: &Options,
        primary_path: P,
        secondary_path: P,
        name: &str,
        cfs: Vec<ColumnFamilyName>,
    ) -> Result<Self, Self::Error>;

    fn log_construct(name: &str) -> Self;

    /// Reads single record by key.
    fn get<S: Schema>(&self, schema_key: &S::Key) -> Result<Option<S::Value>, Self::Error>;

    /// Writes single record.
    fn put<S: Schema>(&self, key: &S::Key, value: &S::Value) -> Result<(), Self::Error>;

    /// Deletes a single record.
    fn delete<S: Schema>(&self, key: &S::Key) -> Result<(), Self::Error>;

    fn iter_with_direction<S: Schema>(
        &self,
        opts: ReadOptions,
        direction: ScanDirection,
    ) -> Result<SchemaIterator<S>, Self::Error>;

    /// Returns a forward [`SchemaIterator`] on a certain schema.
    fn iter<S: Schema>(&self) -> Result<SchemaIterator<S>, Self::Error>;

    /// Returns a forward [`SchemaIterator`] on a certain schema, with non-default ReadOptions
    fn iter_with_opts<S: Schema>(
        &self,
        opts: ReadOptions,
    ) -> Result<SchemaIterator<S>, Self::Error>;

    /// Returns a backward [`SchemaIterator`] on a certain schema.
    fn rev_iter<S: Schema>(&self) -> Result<SchemaIterator<S>, Self::Error>;

    /// Returns a backward [`SchemaIterator`] on a certain schema, with non-default ReadOptions
    fn rev_iter_with_opts<S: Schema>(
        &self,
        opts: ReadOptions,
    ) -> Result<SchemaIterator<S>, Self::Error>;

    /// Writes a group of records wrapped in a [`SchemaBatch`].
    fn write_schemas(&self, batch: SchemaBatch) -> Result<(), Self::Error>;

    fn get_cf_handle(&self, cf_name: &str) -> Result<Self, Self::Error>;

    /// Flushes memtable data. This is only used for testing `get_approximate_sizes_cf` in unit
    /// tests.
    fn flush_cf(&self, cf_name: &str) -> Result<(), Self::Error>;

    fn get_property(&self, cf_name: &str, property_name: &str) -> Result<u64, Self::Error>;

    /// Creates new physical DB checkpoint in directory specified by `path`.
    fn create_checkpoint<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::Error>;
}
