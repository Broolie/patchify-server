struct PackageResolver {
    auth_identity: usize,
}

impl PackageResolver {
    pub fn new(identity: usize) -> Self {
        Self {
            auth_identity: identity,
        }
    }
}
