#![no_std]


#[derive(Clone, Copy)]
pub struct InterruptPlug{}

pub static GLOBAL_INTERRUPT_PLUG: InterruptPlug = InterruptPlug{};

impl <T> BrickCallbackImpl<T> for InterruptPlug { fn call(&mut self) {} }

pub struct InterruptSplitter<T: ?Sized>
{
    pub splitter_ouput_interrupt: Option<* mut [* mut dyn BrickCallbackImpl<T>]>
}

pub trait PeripheralNew <AnyType, Call: BrickCallbackImpl<AnyType>, T, Inst, const NUMB: usize> {
    fn new (inst: Inst) -> & 'static mut T;
    fn _new(inst: Inst, interrupt_arguments: [&mut Call; NUMB]) -> T;
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
    pub fn call_all(&mut self) { unsafe {
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