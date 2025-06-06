//! Helper traits to wrap generic l1 errors, in network specific error type
//! configured in `reth_rpc_eth_api::EthApiTypes`.

use crate::error::EthApiError;

/// Helper trait to wrap core [`EthApiError`].
pub trait FromEthApiError: From<EthApiError> {
    /// Converts from error via [`EthApiError`].
    fn from_eth_err<E>(err: E) -> Self
    where EthApiError: From<E>;
}

impl<T> FromEthApiError for T
where T: From<EthApiError>
{
    fn from_eth_err<E>(err: E) -> Self
    where EthApiError: From<E> {
        T::from(EthApiError::from(err))
    }
}

/// Helper trait to wrap core [`EthApiError`].
pub trait IntoEthApiError: Into<EthApiError> {
    /// Converts into error via [`EthApiError`].
    fn into_eth_err<E>(self) -> E
    where E: FromEthApiError;
}

impl<T> IntoEthApiError for T
where EthApiError: From<T>
{
    fn into_eth_err<E>(self) -> E
    where E: FromEthApiError {
        E::from_eth_err(self)
    }
}

/// Helper trait to access wrapped core error.
pub trait AsEthApiError {
    /// Returns reference to [`EthApiError`], if this an error variant inherited
    /// from core functionality.
    fn as_err(&self) -> Option<&EthApiError>;

    /// Returns `true` if error is
    /// [`RpcInvalidTransactionError::GasTooHigh`].
    fn is_gas_too_high(&self) -> bool {
        if let Some(err) = self.as_err() {
            return err.is_gas_too_high();
        }

        false
    }

    /// Returns `true` if error is
    /// [`RpcInvalidTransactionError::GasTooLow`].
    fn is_gas_too_low(&self) -> bool {
        if let Some(err) = self.as_err() {
            return err.is_gas_too_low();
        }

        false
    }
}

impl AsEthApiError for EthApiError {
    fn as_err(&self) -> Option<&EthApiError> { Some(self) }
}
