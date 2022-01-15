#![no_std]

pub use gestalt_reference_api::interface::GenericInterface;

#[derive(Clone, Copy)]
pub struct InterruptPlug{}

pub static global_interrupt_plug: InterruptPlug = InterruptPlug{};

impl <T> BrickCallbackImpl<T> for InterruptPlug { fn call(&mut self) {} }

pub struct InterruptSplitter<T: ?Sized>
{
    pub splitter_ouput_interrupt: Option<* mut [* mut dyn BrickCallbackImpl<T>]>
}

pub trait PeripheralNew <T_2, CALL: BrickCallbackImpl<T_2>, T, INST, const NUMB: usize> {
    fn new (inst: INST) -> & 'static mut T;
    fn _new(inst: INST, interrupt_arguments: [&mut CALL; NUMB]) -> T;
}

pub trait BrickCallbackImpl<T: ?Sized>{
    fn call(&mut self);
}

pub trait BrickPollImpl
{
    fn poll (&mut self);
}

pub trait SplitterImpl<T> {
    fn set_splitter(&mut self, splitter: * mut [* mut dyn BrickCallbackImpl<T>]);
}

impl <T: ?Sized> InterruptSplitter<T> {
    pub fn set_splitter(&mut self, splitter: * mut [* mut dyn BrickCallbackImpl<T>]){
        self.splitter_ouput_interrupt = Some(splitter);
    }

}

impl <T>InterruptSplitter<T> {
    pub fn call(&mut self) { unsafe {
            (*self.splitter_ouput_interrupt.unwrap()).iter().for_each( |&this| (*this).call());
        }
    }
}

pub trait BrickMultyUsege {
    fn request_access(&mut self);
    fn free(&mut self);
}

#[macro_export]
macro_rules! share
{
    (ptr, $el:ident, $T:ty) =>  { unsafe { (&$el as * const $T) as * mut $T } };
    (mref, $el:ident, $T:ty) => { unsafe { &mut *((&$el as * const $T) as * mut $T) } };
}