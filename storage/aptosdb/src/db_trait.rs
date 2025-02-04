pub trait LDB<D>: Send + Sync {
    /// Opens database with defaults
    fn open(
        path: impl AsRef<Path>,
        name: &str,
        column_families: Vec<ColumnFamilyName>,
        db_opts: &Options,
    ) -> DbResult<D>;

    /// opens database in cf mode
    fn open_cf(
        db_opts: &Options,
        path: impl AsRef<Path>,
        name: &str,
        cfds: Vec<ColumnFamilyDescriptor>,
    ) -> DbResult<D>;

    /// Open db in readonly mode
    /// Note that this still assumes there's only one process that opens the same DB.
    /// See `open_as_secondary`
    pub fn open_cf_readonly(
        opts: &Options,
        path: impl AsRef<Path>,
        name: &str,
        cfs: Vec<ColumnFamilyName>,
    ) -> DbResult<D>;

    pub fn open_cf_as_secondary<P: AsRef<Path>>(
        opts: &Options,
        primary_path: P,
        secondary_path: P,
        name: &str,
        cfs: Vec<ColumnFamilyName>,
    ) -> DbResult<D>;

    fn log_construct(name: &str, inner: rocksdb::DB) -> DB;

    /// Reads single record by key.
    pub fn get<S: Schema>(&self, schema_key: &S::Key) -> DbResult<Option<S::Value>>;

    /// Writes single record.
    pub fn put<S: Schema>(&self, key: &S::Key, value: &S::Value) -> DbResult<()>;

    /// Deletes a single record.
    pub fn delete<S: Schema>(&self, key: &S::Key) -> DbResult<()>;

    fn iter_with_direction<S: Schema>(
        &self,
        opts: ReadOptions,
        direction: ScanDirection,
    ) -> DbResult<SchemaIterator<S>>;

    /// Returns a forward [`SchemaIterator`] on a certain schema.
    pub fn iter<S: Schema>(&self) -> DbResult<SchemaIterator<S>>;

    /// Returns a forward [`SchemaIterator`] on a certain schema, with non-default ReadOptions
    pub fn iter_with_opts<S: Schema>(&self, opts: ReadOptions) -> DbResult<SchemaIterator<S>>;

    /// Returns a backward [`SchemaIterator`] on a certain schema.
    pub fn rev_iter<S: Schema>(&self) -> DbResult<SchemaIterator<S>>;

    /// Returns a backward [`SchemaIterator`] on a certain schema, with non-default ReadOptions
    pub fn rev_iter_with_opts<S: Schema>(&self, opts: ReadOptions) -> DbResult<SchemaIterator<S>>;

    /// Writes a group of records wrapped in a [`SchemaBatch`].
    pub fn write_schemas(&self, batch: SchemaBatch) -> DbResult<()>;

    fn get_cf_handle(&self, cf_name: &str) -> DbResult<&rocksdb::ColumnFamily>;

    /// Flushes memtable data. This is only used for testing `get_approximate_sizes_cf` in unit
    /// tests.
    pub fn flush_cf(&self, cf_name: &str) -> DbResult<()>;

    pub fn get_property(&self, cf_name: &str, property_name: &str) -> DbResult<u64>;

    /// Creates new physical DB checkpoint in directory specified by `path`.
    pub fn create_checkpoint<P: AsRef<Path>>(&self, path: P) -> DbResult<()>;
}
