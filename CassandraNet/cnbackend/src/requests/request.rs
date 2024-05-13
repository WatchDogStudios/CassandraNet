/// Copyright (C) 2024-present WD Studios L.L.C. & CassandraNet Contributors.

pub enum CnRequestType{
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Trace,
    Connect,
    Custom, ///> (Mikael A.) Custom request type for any other request type that is not listed here. could be evaluated as a class of its own.
    Unknown,
}

pub enum Method {
    Get,
}

pub trait CnCustomRequest<T> {
    fn register_custom_request(&self, request_type: T);
}


pub struct CnRequest<T> {
    pub request_id: u64,
    pub request_type: T,
    pub request_data: String, ///> optional
    pub headers: HashMap<String, String>,
}

impl<T> CnRequest<T> for CnCustomRequest<T> {
    fn new(request_id: u64, request_type: T, request_data: String, headers: HashMap<String, String>) -> Self {
        let newrequest = CnRequest {
            request_id,
            request_type,
            request_data,
            headers,
        };
        
    }

    fn register_custom_request(&self, request_type: T) {
        cncommon::CnProfile::start_event("register_custom_request");
        self.request_type = request_type;
    }
}