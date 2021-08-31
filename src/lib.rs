#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]
#![no_std]

//use core::borrow::{BorrowMut, Borrow};
pub use gestalt_reference_api::interface::GenericInterface;
pub enum LockStatus
{
    Unlock,
    Lock{adr: usize}
}

/// Brick body, that include in itself:
/// generic status (u should use enums, but it possible use other entities),
/// Data, that has generic type for storing and working with any data.
/// EXT_FN_NB - number interrupt, that Brick can listen.
/// EXT_EVT_NB - number interrupt, that Brick can give to other Bricks
pub struct BrickBase<STATUS, DATA:Default<DATA>, const EXT_FN_NB: usize,  const EXT_EVT_NB: usize>
{

    pub local_status: STATUS,
    extern_event:   [fn(); EXT_FN_NB],
    own_event:      [fn(); EXT_FN_NB],
    extern_event_fn:[fn(); EXT_EVT_NB],
    lock_status:      LockStatus,
    pub data: DATA
}

//unsafe impl <STATUS,TT:Default<TT>, const EXT_FN_NB: usize>Send for BrickBase<STATUS, TT, EXT_FN_NB>{}
//unsafe impl <STATUS,TT:Default<TT>, const EXT_FN_NB: usize>Sync for BrickBase<STATUS, TT, EXT_FN_NB>{}

pub enum LocalStatus<STATUS>
{
    Interrupt(STATUS),
    NormalWork(STATUS)
}

fn empty_fn(){}

pub trait  Default<STATUS>
{
    fn default() -> STATUS;
}

pub struct Brick <STATUS, DATA:Default<DATA>, const EXT_FN_NB: usize,  const EXT_EVT_NB: usize>
{
    base: BrickBase<STATUS, DATA, EXT_FN_NB, EXT_EVT_NB>
}

pub const fn new <STATUS: Clone, TT:Default<TT>, const EXT_FN_NB: usize, const EXT_EVT_NB: usize>
    (local_event: STATUS, own_transmit_events: [fn(); EXT_FN_NB], data: TT) ->
    Brick<STATUS, TT, EXT_FN_NB, EXT_EVT_NB>
{
    Brick
    {
        base: BrickBase
        {
            lock_status:      LockStatus::Unlock,
            local_status:   local_event,
            extern_event:   own_transmit_events,
            own_event:      own_transmit_events,
            data,
            extern_event_fn: [empty_fn; EXT_EVT_NB]
        }
    }
}

pub trait BrickBaseImpl<STATUS: Clone, DATA:Default<DATA>,
    const EXT_FN_NB: usize, const EXT_EVT_NB: usize>
{
    fn get_l_status(&mut self) -> STATUS;
    fn set_l_status(&mut self, status: STATUS);
    fn get_mut_data(&mut self) -> & mut DATA;
    fn listen_event(&mut self, self_channel: usize, other: &mut BrickBase<STATUS, DATA,
        EXT_FN_NB, EXT_EVT_NB>, other_channel: usize);
    fn confirm_event(&mut self, channel: usize);


}

pub trait BrickMutexImpl<STATUS: Clone, TT:Default<TT>,
    const EXT_FN_NB: usize, const EXT_EVT_NB: usize>
{
    fn try_get_mut_borrow(&mut self) -> Option<& mut BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB>>;
    unsafe fn get_mut_borrow(&mut self) -> & mut BrickBase<STATUS,TT, EXT_FN_NB, EXT_EVT_NB>;
    fn is_lock(&self) -> bool;
}

pub trait BrickExternImpl<STATUS: Clone, TT:Default<TT>, const EXT_FN_NB: usize>
{
    fn brick_main(&mut self);
    fn brick_event(&mut self);
    fn poll (&mut self);

}


impl <STATUS: Clone, TT:Default<TT>, const EXT_FN_NB: usize, const EXT_EVT_NB: usize>
    BrickMutexImpl<STATUS,TT, EXT_FN_NB, EXT_EVT_NB> for Brick<STATUS,TT, EXT_FN_NB, EXT_EVT_NB>
{
    fn try_get_mut_borrow(&mut self) -> Option<& mut BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB>>
    {
        match self.base.lock_status
        {
            LockStatus::Unlock =>
                {
                    self.base.lock_status = LockStatus::Lock {adr: (& mut self.base)
                        as * const BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB> as usize};
                    Some(& mut self.base)
                }
            LockStatus::Lock { adr } =>
                {
                    if adr == (& mut self.base as
                        * const BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB> as usize)
                    { Some(& mut self.base) }
                    else { None } }
        }
    }

    unsafe fn get_mut_borrow(&mut self) -> & mut BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB> {
        & mut self.base
    }

    fn is_lock(&self) -> bool
    {
        match self.base.lock_status
        {
            LockStatus::Unlock => { false }
            LockStatus::Lock { .. } => { true }
        }
    }
}

impl <STATUS: Clone, TT:Default<TT>, const EXT_FN_NB: usize, const EXT_EVT_NB: usize>
    BrickBaseImpl<STATUS, TT, EXT_FN_NB, EXT_EVT_NB> for
    BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB>
{
    fn get_l_status(&mut self) -> STATUS { self.local_status.clone() }

    fn set_l_status(&mut self, status: STATUS) { self.local_status = status; }

    fn get_mut_data(&mut self) -> &mut TT { & mut self.data }

    fn listen_event(&mut self, self_channel: usize, other:
        &mut BrickBase<STATUS, TT, EXT_FN_NB, EXT_EVT_NB>, other_channel: usize)
    {
                self.extern_event[self_channel] = other.extern_event[other_channel];
                other.extern_event[other_channel] = self.own_event[self_channel];
    }

    fn confirm_event(&mut self, channel: usize) { (self.extern_event[channel])(); }


}
