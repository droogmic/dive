
//type DiveError = String;
//
//pub trait LedgerBlock {}
//
//pub trait Secret {}
//
//pub trait OpenSecret {}
//
//pub trait OpenBiometric {}
//
//pub trait LedgerRead {
//    //fn get_block<Q: ?Sized>(&self, key: &Q) -> Option<dyn LedgerBlock>
//    //where
//    //    Self: Sized,
//    //    Q: Hash + Eq,
//    //;
//}
//
//pub trait LedgerWrite {
//    fn add_block(&self, block: dyn LedgerBlock) -> Result<(), DiveError>
//    where
//        Self: Sized,
//    ;
//}
//
//pub trait SecretOwn {
//    //fn request_secret<Q: ?Sized>(&self, key: &Q) -> Option<dyn OpenSecret>
//    //where
//    //    Self: Sized,
//    //    Q: Hash + Eq;
//}
//
//pub trait BiometricCheck {
//    fn check_biometric(&self, biometric: dyn OpenBiometric) -> Result<(), DiveError>
//    where
//        Self: Sized,
//    ;
//}
//
//pub trait SecretWrite {
//    //fn add_secret(&self, secret: dyn Secret) -> Result<dyn OpenSecret, DiveError>
//    //where
//    //    Self: Sized,
//    //;
//}
//
//pub trait SecretCheck {
//    fn check_secret(&self, secret: dyn OpenSecret) -> Result<(), DiveError>
//    where
//        Self: Sized,
//    ;
//}

