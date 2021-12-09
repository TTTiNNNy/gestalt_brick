#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
//#![feature(const_trait_impl)]
#![no_std]

use core::marker::PhantomData;
//use core::borrow::{BorrowMut, Borrow};
pub use gestalt_reference_api::interface::GenericInterface;

/// Brick body, that include in itself:
/// generic status (u should use enums, but it possible use other entities),
/// Data, that has generic type for storing and working with any data.
/// EXT_FN_NB - number interrupt, that Brick can listen.
/// EXT_EVT_NB - number interrupt, that Brick can give to other Bricks

pub struct InterruptPlug{}

impl <T>CallbackExec<T> for InterruptPlug { fn call(&mut self) {} }

pub struct InterruptSplitter<'a, T: ?Sized>
{
    pub splitter_ouput_interrupt: &'a mut [&'a mut T]
}

pub struct BrickBase<'a, T_2, STATUS, DATA, const INTERRUPT_NUMB: usize>
{
    pub status: STATUS,
    pub data: DATA,
    pub output_interrupt: [InterruptSplitter<'a, CallbackExec<T_2>>; INTERRUPT_NUMB]
}


pub trait PeripheralNew <T_2, CALL: CallbackExec<T_2>, T, INST, const NUMB: usize> {
    fn new (inst: INST) -> & 'static mut T;
    fn _new(inst: INST, interrupt_arguments: [&mut CALL; NUMB]) -> T;
}

pub trait CallbackExec<T>{
    fn call(&mut self);
}


pub trait  Default<STATUS>
{
    fn default() -> STATUS;
}

pub trait BrickExternImpl
{
    fn brick_main(&mut self);
    fn poll (&mut self);

}

// impl <STATUS: Clone, DATA>
//     BrickBaseImpl<STATUS, DATA> for
//     BrickBase<STATUS, DATA>
// {
//     fn get_status(&mut self) -> STATUS { self.local_status.clone() }
// }
