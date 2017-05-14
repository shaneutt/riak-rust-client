use errors::RiakErr;
use protobuf::Message;
use rpb::riak_dt::{DtFetchResp, DtFetchResp_DataType, DtFetchReq, DtUpdateReq, DtValue, MapEntry,
                   MapField, MapField_MapFieldType};
use utils::{DataTypeMapEntryPrivate, DataTypeFetchRespPrivate, DataTypeMapFieldPrivate,
            DataTypeValuePrivate, ProtobufBytes};

/// represents a request to store a Data Type object
#[derive(Clone, Debug)]
pub struct DataTypeUpdateReq(DtUpdateReq);

impl DataTypeUpdateReq {
    /// constructs a new `DataTypeUpdateReq`
    pub fn new<T: Into<Vec<u8>>>(bucket: T, bucket_type: T) -> DataTypeUpdateReq {
        let mut req = DtUpdateReq::new();
        req.set_bucket(bucket.into());
        req.set_field_type(bucket_type.into());
        DataTypeUpdateReq(req)
    }
}

/// represents a request to fetch an object via Riak Data Types
#[derive(Clone, Debug)]
pub struct DataTypeFetchReq(DtFetchReq);

impl DataTypeFetchReq {
    /// constructs a new `DataTypeFetchReq`
    pub fn new<T: Into<Vec<u8>>>(bucket_type: T, bucket: T, key: T) -> DataTypeFetchReq {
        let mut req = DtFetchReq::new();
        req.set_bucket(bucket.into());
        req.set_field_type(bucket_type.into());
        req.set_key(key.into());
        DataTypeFetchReq(req)
    }

    /// get the "bucket_type" property
    pub fn get_bucket_type(&self) -> &[u8] {
        self.0.get_field_type()
    }

    /// set the "bucket_type" property
    pub fn set_bucket_type<T: Into<Vec<u8>>>(&mut self, bucket_type: T) {
        self.0.set_field_type(bucket_type.into());
    }

    /// get the "bucket" property
    pub fn get_bucket(&self) -> &[u8] {
        self.0.get_bucket()
    }

    /// set the "bucket" property
    pub fn set_bucket<T: Into<Vec<u8>>>(&mut self, bucket: T) {
        self.0.set_bucket(bucket.into());
    }

    /// get the "key" property
    pub fn get_key(&self) -> &[u8] {
        self.0.get_key()
    }

    /// set the "key" property
    pub fn set_key<T: Into<Vec<u8>>>(&mut self, key: T) {
        self.0.set_key(key.into());
    }

    /// get the "r" property
    pub fn get_r(&self) -> Option<u32> {
        if self.0.has_r() {
            Some(self.0.get_r())
        } else {
            None
        }
    }

    /// set the "r" property
    pub fn set_r(&mut self, r: u32) {
        self.0.set_r(r);
    }

    /// get the "pr" property
    pub fn get_pr(&self) -> Option<u32> {
        if self.0.has_pr() {
            Some(self.0.get_pr())
        } else {
            None
        }
    }

    /// set the "pr" property
    pub fn set_pr(&mut self, pr: u32) {
        self.0.set_pr(pr);
    }

    /// get the "basic_quorum" property
    pub fn get_basic_quorum(&self) -> Option<bool> {
        if self.0.has_basic_quorum() {
            Some(self.0.get_basic_quorum())
        } else {
            None
        }
    }

    /// set the "basic_quorum" property
    pub fn set_basic_quorum(&mut self, basic_quorum: bool) {
        self.0.set_basic_quorum(basic_quorum);
    }

    /// get the "notfound_ok" property
    pub fn get_notfound_ok(&self) -> Option<bool> {
        if self.0.has_notfound_ok() {
            Some(self.0.get_notfound_ok())
        } else {
            None
        }
    }

    /// set the "notfound_ok" property
    pub fn set_notfound_ok(&mut self, notfound_ok: bool) {
        self.0.set_notfound_ok(notfound_ok);
    }

    /// get the "timeout" property
    pub fn get_timeout(&self) -> Option<u32> {
        if self.0.has_timeout() {
            Some(self.0.get_timeout())
        } else {
            None
        }
    }

    /// set the "timeout" property
    pub fn set_timeout(&mut self, timeout: u32) {
        self.0.set_timeout(timeout);
    }

    /// get the "sloppy_quorum" property
    pub fn get_sloppy_quorum(&self) -> Option<bool> {
        if self.0.has_sloppy_quorum() {
            Some(self.0.get_sloppy_quorum())
        } else {
            None
        }
    }

    /// set the "sloppy_quorum" property
    pub fn set_sloppy_quorum(&mut self, sloppy_quorum: bool) {
        self.0.set_sloppy_quorum(sloppy_quorum);
    }

    /// get the "n_val" property
    pub fn get_n_val(&self) -> Option<u32> {
        if self.0.has_n_val() {
            Some(self.0.get_n_val())
        } else {
            None
        }
    }

    /// set the "n_val" property
    pub fn set_n_val(&mut self, n_val: u32) {
        self.0.set_n_val(n_val);
    }

    /// get the "include_context" property
    pub fn get_include_context(&self) -> Option<bool> {
        if self.0.has_include_context() {
            Some(self.0.get_include_context())
        } else {
            None
        }
    }

    /// set the "include_context" property
    pub fn set_include_context(&mut self, include_context: bool) {
        self.0.set_include_context(include_context);
    }
}

impl ProtobufBytes for DataTypeFetchReq {
    fn write_to_bytes(self) -> Result<Vec<u8>, RiakErr> {
        match self.0.write_to_bytes() {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(RiakErr::ProtobufError(e)),
        }
    }
}

/// represents the valid data types
#[derive(Clone, Debug)]
pub enum DataType {
    COUNTER,
    SET,
    MAP,
    HLL,
}

/// represents the response from a `DTFetchReq`
#[derive(Clone, Debug)]
pub struct DataTypeFetchResp {
    pub context: Option<Vec<u8>>,
    pub data_type: DataType,
    pub value: Option<DataTypeValue>,
}

impl DataTypeFetchRespPrivate for DataTypeFetchResp {
    fn new_from_rpb(mut dt_fetch_resp: DtFetchResp) -> DataTypeFetchResp {
        // get the context
        let mut context: Option<Vec<u8>> = None;
        if dt_fetch_resp.has_context() {
            context = Some(dt_fetch_resp.take_context());
        }

        // get the data_type
        let data_type = match dt_fetch_resp.get_field_type() {
            DtFetchResp_DataType::COUNTER => DataType::COUNTER,
            DtFetchResp_DataType::SET => DataType::SET,
            DtFetchResp_DataType::MAP => DataType::MAP,
            DtFetchResp_DataType::HLL => DataType::HLL,
        };

        // get the value
        let mut value: Option<DataTypeValue> = None;
        if dt_fetch_resp.has_value() {
            value = Some(DataTypeValue::new_from_rpb(dt_fetch_resp.take_value()));
        }

        DataTypeFetchResp {
            context: context,
            data_type: data_type,
            value: value,
        }
    }
}

/// represents a map entry
#[derive(Clone, Debug)]
pub struct DataTypeMapEntry {
    pub field: DataTypeMapField,
    pub counter_value: Option<i64>,
    pub set_value: Vec<Vec<u8>>,
    pub register_value: Option<Vec<u8>>,
    pub flag_value: Option<bool>,
    pub map_value: Vec<DataTypeMapEntry>,
}

impl DataTypeMapEntryPrivate for DataTypeMapEntry {
    fn new_from_rpb(mut rpb_map_entry: MapEntry) -> DataTypeMapEntry {
        // get the field
        let field = DataTypeMapField::new_from_rpb(rpb_map_entry.take_field());

        // get the counter_value
        let mut counter_value: Option<i64> = None;
        if rpb_map_entry.has_counter_value() {
            counter_value = Some(rpb_map_entry.get_counter_value());
        }

        // get the set_value
        let mut set_value: Vec<Vec<u8>> = Vec::new();
        for next_set_value in rpb_map_entry.take_set_value().into_iter() {
            set_value.push(next_set_value);
        }

        // get the register_value
        let mut register_value: Option<Vec<u8>> = None;
        if rpb_map_entry.has_register_value() {
            register_value = Some(rpb_map_entry.take_register_value());
        }

        // get the flag_value
        let mut flag_value: Option<bool> = None;
        if rpb_map_entry.has_flag_value() {
            flag_value = Some(rpb_map_entry.get_flag_value());
        }

        // get the map_value
        let mut map_value = Vec::new();
        for next_rpb_map_entry in rpb_map_entry.take_map_value().into_iter() {
            map_value.push(DataTypeMapEntry::new_from_rpb(next_rpb_map_entry));
        }

        DataTypeMapEntry {
            field: field,
            counter_value: counter_value,
            set_value: set_value,
            register_value: register_value,
            flag_value: flag_value,
            map_value: map_value,
        }
    }
}

/// represents the map field
#[derive(Clone, Debug)]
pub struct DataTypeMapField {
    pub name: Vec<u8>,
    pub field_type: DataTypeMapFieldType,
}

impl DataTypeMapFieldPrivate for DataTypeMapField {
    fn new_from_rpb(mut rpb_map_field: MapField) -> DataTypeMapField {
        let field_type = match rpb_map_field.get_field_type() {
            MapField_MapFieldType::COUNTER => DataTypeMapFieldType::COUNTER,
            MapField_MapFieldType::SET => DataTypeMapFieldType::SET,
            MapField_MapFieldType::REGISTER => DataTypeMapFieldType::REGISTER,
            MapField_MapFieldType::FLAG => DataTypeMapFieldType::FLAG,
            MapField_MapFieldType::MAP => DataTypeMapFieldType::MAP,
        };

        DataTypeMapField {
            name: rpb_map_field.take_name(),
            field_type: field_type,
        }
    }
}

/// represents the types valid for a map field
#[derive(Clone, Debug)]
pub enum DataTypeMapFieldType {
    COUNTER,
    SET,
    REGISTER,
    FLAG,
    MAP,
}

/// represents the value of a data-type
#[derive(Clone, Debug)]
pub struct DataTypeValue {
    pub counter_value: Option<i64>,
    pub set_value: Vec<Vec<u8>>,
    pub map_value: Vec<DataTypeMapEntry>,
}

impl DataTypeValuePrivate for DataTypeValue {
    fn new_from_rpb(mut dt_value: DtValue) -> DataTypeValue {
        // get the map_value if present
        let mut map_value: Vec<DataTypeMapEntry> = Vec::new();
        for rpb_map_entry in dt_value.take_map_value().into_iter() {
            let map_entry = DataTypeMapEntry::new_from_rpb(rpb_map_entry);
            map_value.push(map_entry);
        }

        // get the counter_value if present
        let mut counter_value: Option<i64> = None;
        if dt_value.has_counter_value() {
            counter_value = Some(dt_value.get_counter_value());
        }

        // get the set_value
        let set_value = dt_value.take_set_value();

        DataTypeValue {
            counter_value: counter_value,
            set_value: set_value.to_vec(),
            map_value: map_value,
        }
    }
}
