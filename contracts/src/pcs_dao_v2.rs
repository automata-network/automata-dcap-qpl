use ethers::contract::abigen;

// Exact-index V2 keeps the existing PCS upsert ABI. The helper marker detects
// the eager implementation, while indexedCrls verifies that the same upsert
// transaction made the exact DER ready for O(1) membership checks.
abigen!(
    X509CrlHelperV2,
    r#"[
        function getTbsAndSig(bytes der) external pure returns (bytes tbs, bytes sig)
        function crlRevokedSetHashes(bytes32 derHash) external view returns (bytes32 revokedSetHash)
        function indexedCrls(bytes32 derHash) external view returns (bool)
    ]"#,
);
