#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct InitMsg {
    /// initial allowlist
    pub allowlist: Vec<AllowMsg>,
    /// If set, contracts off the allowlist will run with this gas limit.
    /// If unset, will refuse to accept any contract off the allow list.
    pub default_gas_limit: Option<u64>,
}