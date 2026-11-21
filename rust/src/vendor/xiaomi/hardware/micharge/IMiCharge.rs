/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/setsuneko/DSH-Workgroup/BakaBuildPlafrom/ACK-6.12/prebuilts/build-tools/linux-x86/bin/aidl --lang=rust -I aidl -o gen/rust aidl/vendor/xiaomi/hardware/micharge/IMiCharge.aidl
 *
 * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
 * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
 * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
#[cfg(any(android_vndk, not(android_ndk)))]
const FLAG_PRIVATE_LOCAL: binder::binder_impl::TransactionFlags = binder::binder_impl::FLAG_PRIVATE_LOCAL;
#[cfg(not(any(android_vndk, not(android_ndk))))]
const FLAG_PRIVATE_LOCAL: binder::binder_impl::TransactionFlags = 0;
use binder::declare_binder_interface;
declare_binder_interface! {
  IMiCharge["vendor.xiaomi.hardware.micharge.IMiCharge"] {
    native: BnMiCharge(on_transact),
    proxy: BpMiCharge {
    },
    async: IMiChargeAsync(try_into_local_async),
  }
}
pub trait IMiCharge: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "vendor.xiaomi.hardware.micharge.IMiCharge" }
  fn r#getQuickChargeType<'a, >(&'a self) -> binder::Result<String>;
  fn r#getTxAdapt<'a, >(&'a self) -> binder::Result<String>;
  fn r#getChargingPowerMax<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryChargeType<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryVbat<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryIbat<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryCapacity<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatterySoh<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryTbat<'a, >(&'a self) -> binder::Result<String>;
  fn r#isFunctionSupported<'a, 'l1, >(&'a self, _arg_funcName: &'l1 str) -> binder::Result<bool>;
  fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::Result<bool>;
  fn r#getMiChargePath<'a, 'l1, >(&'a self, _arg_path: &'l1 str) -> binder::Result<String>;
  fn r#getBatteryCommonInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 str) -> binder::Result<String>;
  fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryResistance<'a, >(&'a self) -> binder::Result<String>;
  fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::Result<String>;
  fn getDefaultImpl() -> IMiChargeDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IMiChargeDefaultRef) -> IMiChargeDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
  fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IMiChargeAsyncServer + Send + Sync)> {
    None
  }
}
pub trait IMiChargeAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "vendor.xiaomi.hardware.micharge.IMiCharge" }
  fn r#getQuickChargeType<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getTxAdapt<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getChargingPowerMax<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryChargeType<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryVbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryIbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryCapacity<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatterySoh<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryTbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#isFunctionSupported<'a, >(&'a self, _arg_funcName: &'a str) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#getMiChargePath<'a, >(&'a self, _arg_path: &'a str) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryCommonInfo<'a, >(&'a self, _arg_info: &'a str) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryResistance<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>>;
}
#[::async_trait::async_trait]
pub trait IMiChargeAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "vendor.xiaomi.hardware.micharge.IMiCharge" }
  async fn r#getQuickChargeType<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getTxAdapt<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getChargingPowerMax<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryChargeType<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryVbat<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryIbat<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryCapacity<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatterySoh<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryTbat<'a, >(&'a self) -> binder::Result<String>;
  async fn r#isFunctionSupported<'a, 'l1, >(&'a self, _arg_funcName: &'l1 str) -> binder::Result<bool>;
  async fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::Result<bool>;
  async fn r#getMiChargePath<'a, 'l1, >(&'a self, _arg_path: &'l1 str) -> binder::Result<String>;
  async fn r#getBatteryCommonInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 str) -> binder::Result<String>;
  async fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryResistance<'a, >(&'a self) -> binder::Result<String>;
  async fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::Result<String>;
}
impl BnMiCharge {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IMiCharge>
  where
    T: IMiChargeAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IMiCharge for Wrapper<T, R>
    where
      T: IMiChargeAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#getQuickChargeType<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getQuickChargeType())
      }
      fn r#getTxAdapt<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getTxAdapt())
      }
      fn r#getChargingPowerMax<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getChargingPowerMax())
      }
      fn r#getBatteryChargeType<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryChargeType())
      }
      fn r#getBatteryVbat<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryVbat())
      }
      fn r#getBatteryIbat<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryIbat())
      }
      fn r#getBatteryCapacity<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryCapacity())
      }
      fn r#getBatterySoh<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatterySoh())
      }
      fn r#getBatteryTbat<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryTbat())
      }
      fn r#isFunctionSupported<'a, 'l1, >(&'a self, _arg_funcName: &'l1 str) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#isFunctionSupported(_arg_funcName))
      }
      fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#isWirelessChargingSupported())
      }
      fn r#getMiChargePath<'a, 'l1, >(&'a self, _arg_path: &'l1 str) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getMiChargePath(_arg_path))
      }
      fn r#getBatteryCommonInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 str) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryCommonInfo(_arg_info))
      }
      fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryAuthentic())
      }
      fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryChargeFull())
      }
      fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryCycleCount())
      }
      fn r#getBatteryResistance<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryResistance())
      }
      fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getBatteryThermaLevel())
      }
      fn try_as_async_server(&self) -> Option<&(dyn IMiChargeAsyncServer + Send + Sync)> {
        Some(&self._inner)
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
  pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IMiChargeAsync<P>>> {
    struct Wrapper {
      _native: binder::binder_impl::Binder<BnMiCharge>
    }
    impl binder::Interface for Wrapper {}
    impl<P: binder::BinderAsyncPool> IMiChargeAsync<P> for Wrapper {
      fn r#getQuickChargeType<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getQuickChargeType())
      }
      fn r#getTxAdapt<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getTxAdapt())
      }
      fn r#getChargingPowerMax<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getChargingPowerMax())
      }
      fn r#getBatteryChargeType<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryChargeType())
      }
      fn r#getBatteryVbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryVbat())
      }
      fn r#getBatteryIbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryIbat())
      }
      fn r#getBatteryCapacity<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryCapacity())
      }
      fn r#getBatterySoh<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatterySoh())
      }
      fn r#getBatteryTbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryTbat())
      }
      fn r#isFunctionSupported<'a, >(&'a self, _arg_funcName: &'a str) -> binder::BoxFuture<'a, binder::Result<bool>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#isFunctionSupported(_arg_funcName))
      }
      fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#isWirelessChargingSupported())
      }
      fn r#getMiChargePath<'a, >(&'a self, _arg_path: &'a str) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getMiChargePath(_arg_path))
      }
      fn r#getBatteryCommonInfo<'a, >(&'a self, _arg_info: &'a str) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryCommonInfo(_arg_info))
      }
      fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryAuthentic())
      }
      fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryChargeFull())
      }
      fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryCycleCount())
      }
      fn r#getBatteryResistance<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryResistance())
      }
      fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
        Box::pin(self._native.try_as_async_server().unwrap().r#getBatteryThermaLevel())
      }
    }
    if _native.try_as_async_server().is_some() {
      Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IMiChargeAsync<P>>))
    } else {
      None
    }
  }
}
pub trait IMiChargeDefault: Send + Sync {
  fn r#getQuickChargeType<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getTxAdapt<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getChargingPowerMax<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryChargeType<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryVbat<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryIbat<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryCapacity<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatterySoh<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryTbat<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#isFunctionSupported<'a, 'l1, >(&'a self, _arg_funcName: &'l1 str) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getMiChargePath<'a, 'l1, >(&'a self, _arg_path: &'l1 str) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryCommonInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 str) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryResistance<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#getQuickChargeType: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#getTxAdapt: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#getChargingPowerMax: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#getBatteryChargeType: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#getBatteryVbat: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#getBatteryIbat: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
  pub const r#getBatteryCapacity: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
  pub const r#getBatterySoh: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
  pub const r#getBatteryTbat: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
  pub const r#isFunctionSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
  pub const r#isWirelessChargingSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
  pub const r#getMiChargePath: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
  pub const r#getBatteryCommonInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
  pub const r#getBatteryAuthentic: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
  pub const r#getBatteryChargeFull: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
  pub const r#getBatteryCycleCount: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
  pub const r#getBatteryResistance: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
  pub const r#getBatteryThermaLevel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
}
pub type IMiChargeDefaultRef = Option<std::sync::Arc<dyn IMiChargeDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IMiChargeDefaultRef> = std::sync::Mutex::new(None);
impl BpMiCharge {
  fn build_parcel_getQuickChargeType(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getQuickChargeType(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getQuickChargeType();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getTxAdapt(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getTxAdapt(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getTxAdapt();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getChargingPowerMax(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getChargingPowerMax(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getChargingPowerMax();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryChargeType(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryChargeType(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryChargeType();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryVbat(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryVbat(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryVbat();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryIbat(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryIbat(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryIbat();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryCapacity(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryCapacity(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryCapacity();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatterySoh(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatterySoh(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatterySoh();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryTbat(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryTbat(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryTbat();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_isFunctionSupported(&self, _arg_funcName: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_funcName)?;
    Ok(aidl_data)
  }
  fn read_response_isFunctionSupported(&self, _arg_funcName: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#isFunctionSupported(_arg_funcName);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_isWirelessChargingSupported(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_isWirelessChargingSupported(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#isWirelessChargingSupported();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getMiChargePath(&self, _arg_path: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_path)?;
    Ok(aidl_data)
  }
  fn read_response_getMiChargePath(&self, _arg_path: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getMiChargePath(_arg_path);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryCommonInfo(&self, _arg_info: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_info)?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryCommonInfo(&self, _arg_info: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryCommonInfo(_arg_info);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryAuthentic(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryAuthentic(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryAuthentic();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryChargeFull(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryChargeFull(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryChargeFull();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryCycleCount(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryCycleCount(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryCycleCount();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryResistance(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryResistance(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryResistance();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getBatteryThermaLevel(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getBatteryThermaLevel(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IMiCharge>::getDefaultImpl() {
        return _aidl_default_impl.r#getBatteryThermaLevel();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IMiCharge for BpMiCharge {
  fn r#getQuickChargeType<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getQuickChargeType()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getQuickChargeType, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getQuickChargeType(_aidl_reply)
  }
  fn r#getTxAdapt<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getTxAdapt()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getTxAdapt, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getTxAdapt(_aidl_reply)
  }
  fn r#getChargingPowerMax<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getChargingPowerMax()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getChargingPowerMax, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getChargingPowerMax(_aidl_reply)
  }
  fn r#getBatteryChargeType<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryChargeType()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryChargeType, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryChargeType(_aidl_reply)
  }
  fn r#getBatteryVbat<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryVbat()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryVbat, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryVbat(_aidl_reply)
  }
  fn r#getBatteryIbat<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryIbat()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryIbat, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryIbat(_aidl_reply)
  }
  fn r#getBatteryCapacity<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryCapacity()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryCapacity, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryCapacity(_aidl_reply)
  }
  fn r#getBatterySoh<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatterySoh()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatterySoh, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatterySoh(_aidl_reply)
  }
  fn r#getBatteryTbat<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryTbat()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryTbat, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryTbat(_aidl_reply)
  }
  fn r#isFunctionSupported<'a, 'l1, >(&'a self, _arg_funcName: &'l1 str) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_isFunctionSupported(_arg_funcName)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#isFunctionSupported, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_isFunctionSupported(_arg_funcName, _aidl_reply)
  }
  fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_isWirelessChargingSupported()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#isWirelessChargingSupported, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_isWirelessChargingSupported(_aidl_reply)
  }
  fn r#getMiChargePath<'a, 'l1, >(&'a self, _arg_path: &'l1 str) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getMiChargePath(_arg_path)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getMiChargePath, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getMiChargePath(_arg_path, _aidl_reply)
  }
  fn r#getBatteryCommonInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 str) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryCommonInfo(_arg_info)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryCommonInfo, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryCommonInfo(_arg_info, _aidl_reply)
  }
  fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryAuthentic()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryAuthentic, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryAuthentic(_aidl_reply)
  }
  fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryChargeFull()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryChargeFull, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryChargeFull(_aidl_reply)
  }
  fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryCycleCount()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryCycleCount, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryCycleCount(_aidl_reply)
  }
  fn r#getBatteryResistance<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryResistance()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryResistance, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryResistance(_aidl_reply)
  }
  fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getBatteryThermaLevel()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getBatteryThermaLevel, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getBatteryThermaLevel(_aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IMiChargeAsync<P> for BpMiCharge {
  fn r#getQuickChargeType<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getQuickChargeType() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getQuickChargeType, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getQuickChargeType(_aidl_reply)
      }
    )
  }
  fn r#getTxAdapt<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getTxAdapt() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getTxAdapt, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getTxAdapt(_aidl_reply)
      }
    )
  }
  fn r#getChargingPowerMax<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getChargingPowerMax() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getChargingPowerMax, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getChargingPowerMax(_aidl_reply)
      }
    )
  }
  fn r#getBatteryChargeType<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryChargeType() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryChargeType, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryChargeType(_aidl_reply)
      }
    )
  }
  fn r#getBatteryVbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryVbat() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryVbat, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryVbat(_aidl_reply)
      }
    )
  }
  fn r#getBatteryIbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryIbat() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryIbat, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryIbat(_aidl_reply)
      }
    )
  }
  fn r#getBatteryCapacity<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryCapacity() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryCapacity, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryCapacity(_aidl_reply)
      }
    )
  }
  fn r#getBatterySoh<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatterySoh() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatterySoh, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatterySoh(_aidl_reply)
      }
    )
  }
  fn r#getBatteryTbat<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryTbat() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryTbat, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryTbat(_aidl_reply)
      }
    )
  }
  fn r#isFunctionSupported<'a, >(&'a self, _arg_funcName: &'a str) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_isFunctionSupported(_arg_funcName) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#isFunctionSupported, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_isFunctionSupported(_arg_funcName, _aidl_reply)
      }
    )
  }
  fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_isWirelessChargingSupported() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#isWirelessChargingSupported, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_isWirelessChargingSupported(_aidl_reply)
      }
    )
  }
  fn r#getMiChargePath<'a, >(&'a self, _arg_path: &'a str) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getMiChargePath(_arg_path) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getMiChargePath, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getMiChargePath(_arg_path, _aidl_reply)
      }
    )
  }
  fn r#getBatteryCommonInfo<'a, >(&'a self, _arg_info: &'a str) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryCommonInfo(_arg_info) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryCommonInfo, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryCommonInfo(_arg_info, _aidl_reply)
      }
    )
  }
  fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryAuthentic() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryAuthentic, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryAuthentic(_aidl_reply)
      }
    )
  }
  fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryChargeFull() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryChargeFull, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryChargeFull(_aidl_reply)
      }
    )
  }
  fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryCycleCount() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryCycleCount, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryCycleCount(_aidl_reply)
      }
    )
  }
  fn r#getBatteryResistance<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryResistance() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryResistance, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryResistance(_aidl_reply)
      }
    )
  }
  fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getBatteryThermaLevel() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getBatteryThermaLevel, _aidl_data, FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getBatteryThermaLevel(_aidl_reply)
      }
    )
  }
}
impl IMiCharge for binder::binder_impl::Binder<BnMiCharge> {
  fn r#getQuickChargeType<'a, >(&'a self) -> binder::Result<String> { self.0.r#getQuickChargeType() }
  fn r#getTxAdapt<'a, >(&'a self) -> binder::Result<String> { self.0.r#getTxAdapt() }
  fn r#getChargingPowerMax<'a, >(&'a self) -> binder::Result<String> { self.0.r#getChargingPowerMax() }
  fn r#getBatteryChargeType<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryChargeType() }
  fn r#getBatteryVbat<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryVbat() }
  fn r#getBatteryIbat<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryIbat() }
  fn r#getBatteryCapacity<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryCapacity() }
  fn r#getBatterySoh<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatterySoh() }
  fn r#getBatteryTbat<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryTbat() }
  fn r#isFunctionSupported<'a, 'l1, >(&'a self, _arg_funcName: &'l1 str) -> binder::Result<bool> { self.0.r#isFunctionSupported(_arg_funcName) }
  fn r#isWirelessChargingSupported<'a, >(&'a self) -> binder::Result<bool> { self.0.r#isWirelessChargingSupported() }
  fn r#getMiChargePath<'a, 'l1, >(&'a self, _arg_path: &'l1 str) -> binder::Result<String> { self.0.r#getMiChargePath(_arg_path) }
  fn r#getBatteryCommonInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 str) -> binder::Result<String> { self.0.r#getBatteryCommonInfo(_arg_info) }
  fn r#getBatteryAuthentic<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryAuthentic() }
  fn r#getBatteryChargeFull<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryChargeFull() }
  fn r#getBatteryCycleCount<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryCycleCount() }
  fn r#getBatteryResistance<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryResistance() }
  fn r#getBatteryThermaLevel<'a, >(&'a self) -> binder::Result<String> { self.0.r#getBatteryThermaLevel() }
}
fn on_transact(_aidl_service: &dyn IMiCharge, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#getQuickChargeType => {
      let _aidl_return = _aidl_service.r#getQuickChargeType();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getTxAdapt => {
      let _aidl_return = _aidl_service.r#getTxAdapt();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getChargingPowerMax => {
      let _aidl_return = _aidl_service.r#getChargingPowerMax();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryChargeType => {
      let _aidl_return = _aidl_service.r#getBatteryChargeType();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryVbat => {
      let _aidl_return = _aidl_service.r#getBatteryVbat();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryIbat => {
      let _aidl_return = _aidl_service.r#getBatteryIbat();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryCapacity => {
      let _aidl_return = _aidl_service.r#getBatteryCapacity();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatterySoh => {
      let _aidl_return = _aidl_service.r#getBatterySoh();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryTbat => {
      let _aidl_return = _aidl_service.r#getBatteryTbat();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#isFunctionSupported => {
      let _arg_funcName: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#isFunctionSupported(&_arg_funcName);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#isWirelessChargingSupported => {
      let _aidl_return = _aidl_service.r#isWirelessChargingSupported();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getMiChargePath => {
      let _arg_path: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getMiChargePath(&_arg_path);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryCommonInfo => {
      let _arg_info: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getBatteryCommonInfo(&_arg_info);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryAuthentic => {
      let _aidl_return = _aidl_service.r#getBatteryAuthentic();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryChargeFull => {
      let _aidl_return = _aidl_service.r#getBatteryChargeFull();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryCycleCount => {
      let _aidl_return = _aidl_service.r#getBatteryCycleCount();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryResistance => {
      let _aidl_return = _aidl_service.r#getBatteryResistance();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getBatteryThermaLevel => {
      let _aidl_return = _aidl_service.r#getBatteryThermaLevel();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IMiCharge as _6_vendor_6_xiaomi_8_hardware_8_micharge_9_IMiCharge;
}
