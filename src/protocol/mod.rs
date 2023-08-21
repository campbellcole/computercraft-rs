macro_rules! define_protocol {
    ($(
        $variant:ident = $request_ty:tt => $response_ty:tt;
    )*) => {
        #[derive(Debug, Clone, Serialize)]
        #[serde(tag = "kind", content = "data")]
        pub enum CCRequestKind {
            $(
                $variant $request_ty,
            )*
        }

        #[derive(Debug, Clone, Serialize)]
        #[serde(tag = "kind", content = "data")]
        pub enum CCResponseKind {
            $(
                $variant $response_ty,
            )*
            Disconnected,
        }
    };
}

define_protocol! {
    Echo = (String) => (String);
    ConnectPeripheral = (String) => (bool);
    CallPeripheral = {
        address: String,
        method: String,
        args: serde_json::Value,
    } => {
        success: bool,
        error: Option<Vec<serde_json::Value>>,
        result: Option<Vec<serde_json::Value>>,
    };
    GetPeripheralType = (String) => (String);
}
