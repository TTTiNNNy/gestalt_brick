#![feature(const_trait_impl)]
use Sync;
use gestalt_brick::{BrickBase, BrickBaseImpl, LocalStatus};


mod uart_mod
{

    use gestalt_reference_api::uart::GestaltUart;
    use gestalt_reference_api::interface::GenericInterface;
    use gestalt_brick::{BrickBase, Default, BrickBaseImpl, BrickMutexImpl, Brick};
    use std::{time, thread};

    use Sync;
    use std::borrow::BorrowMut;
    use crate::gps_mod::GPS;

    #[derive(Clone, Copy)]
    pub enum UartEvent
    {
        Send,
        Receive,
        Init,
        AsyncSend,
        AsyncReceive,
        Idle

    }


    pub struct UartData
    {
        tx: [u8;10],
        rx: [u8;10]
    }

    impl const gestalt_brick::Default<UartData> for UartData
    {
        fn default() -> UartData
        {
            UartData{
                tx: [0;10],
                rx: [0;10]
            }
        }
    }

    impl gestalt_brick::BrickExternImpl<UartEvent, UartData, 1> for BrickBase<UartEvent, UartData, 1, 1>
    {
        fn brick_main(&mut self)
        {
            let delay = time::Duration::from_secs(5);
            let mut ur1 = unsafe { UART1.get_mut_borrow() };


            match ur1.get_l_status()
            {
                UartEvent::Send =>
                    {
                        println!("Uart layer: send data");
                        thread::sleep(delay);
                        println!("Uart layer:Done");
                        ur1.set_l_status(UartEvent::Idle);
                        //UART1.local_status = UartEvent::Init;
                    }
                UartEvent::Receive =>
                    {
                        println!("Uart layer: receive data");
                        thread::sleep(delay);
                        println!("Uart layer:Done");
                        ur1.set_l_status(UartEvent::Idle);

                    }
                UartEvent::Init =>
                    {
                        unsafe { GPS.get_mut_borrow(); }
                        println!("Uart layer: init  data");
                        thread::sleep(delay);
                        println!("Uart layer:Done");
                        ur1.set_l_status(UartEvent::Idle);
                    }
                UartEvent::AsyncSend =>
                    {
                        thread::spawn(move ||
                            {
                                println!("Uart layer: Async Send  data");
                                thread::sleep(delay);
                                println!("Uart layer:Done");
                                ur1.set_l_status(UartEvent::Idle);

                            });
                    }
                UartEvent::AsyncReceive =>
                    {
                        thread::spawn(move ||
                            {
                                println!("Uart layer: Async Read  data");
                                thread::sleep(delay);
                                println!("Uart layer: Done");
                                ur1.set_l_status(UartEvent::Idle);
                            });
                    }
                _ => {}
            }
        }

        fn brick_event(&mut self)
        {

        }

        fn poll(&mut self) { self.brick_main(); }
    }

    impl gestalt_brick::GenericInterface for Brick<UartEvent, UartData,0,2>
    {
        fn write(&mut self) {
            todo!()
        }

        fn read(&mut self) {
            todo!()
        }
    }

    pub static  mut UART1: Brick<UartEvent, UartData,0,2>   =  gestalt_brick::new(UartEvent::Init, [], UartData::default());

}

mod hc05_mod
{
    use gestalt_brick::{BrickBase, Default, Brick};

    #[derive(Clone, Copy)]
    pub enum HCMode
    {
        BLe,
        Bluetooth
    }

    #[derive(Clone, Copy)]
    pub enum HC05Event
    {
        Disconnect,
        Connect,
        Send,
        Receive,
        Init,
        ChangeMode(HCMode)
    }

    fn hc_05_main(){}
    fn hc_05_event(){}

    pub struct HC05Data
    {
        tx:     [u8;10],
        rx:     [u8;10],
        name:   [u8;10],
        connect_type: HCMode
    }

    impl const gestalt_brick::Default<HC05Data> for HC05Data
    {
        fn default() -> HC05Data
        {
            HC05Data
            {
                tx: [0;10],
                rx: [0;10],
                name: [0;10],
                connect_type: HCMode::BLe
            }
        }
    }

    pub static HC05:    Brick<HC05Event, HC05Data, 1, 2> = gestalt_brick::new(HC05Event::Init, [hc_05_event], HC05Data::default());

}

mod gps_mod
{
    use gestalt_brick::{BrickBase, Default, Brick};

    #[derive(Clone, Copy)]
    pub enum GPSvent
    {
        StartGetInfo,
        Receiving,
        Init
    }

    fn gps_event(){}
    fn gps_main(){}

    pub struct GPSData
    {
        latitude: i16,
        longitude: i16
    }

    impl const gestalt_brick::Default<GPSData> for GPSData
    {
        fn default() -> GPSData
        {
            GPSData
            {
                latitude: 0,
                longitude: 0
            }
        }
    }

    pub static mut GPS:    Brick<GPSvent, GPSData, 1, 1> = gestalt_brick::new(GPSvent::Init, [gps_event], GPSData::default());
}

mod main_mod
{
    use crate::gps_mod::GPS;
    pub enum MainEvent
    {
        Connect,
        Disconnect,
        GetGPSData,
        GetSetUpGPGInfo,
        SetGPS,
        SendGPSData,
        Init
    }

    fn main_event(){}
    fn main_main()
    {

    }

    struct MainData
    {

    }

}







//static  HC_05: Brick<HC05Event, HC05Data>   =  gestalt_brick::new(HC05Event::Init, hc_05_main, hc_05_event, Default::default());
//static  GPS: Brick<GPSvent, GPSData>        =  gestalt_brick::new(GPSvent::Init, gps_main, gps_event, Default::default());
//static  MAIN: Brick<MainEvent, MainData>    =  gestalt_brick::new(MainEvent::Init, main_main, main_event, Default::default());


//static UART1:   Brick<UartEvent, > = Brick::<UartEvent>{executor: uart_exec, global_status: GlobalStatus::Idle, local_status: UartEvent::Idle};
//static HC05:    Brick<HC05Event> = Brick::<HC05Event>{executor: hc_05_fn, global_status: GlobalStatus::Idle, local_status: HC05Event::Idle};
//static GPS:     Brick<GPSvent> = Brick::<GPSvent>{executor: hc_05_fn, global_status: GlobalStatus::Idle, local_status: GPSvent::Idle};


fn main ()
{
    println!("q\
    ");
}