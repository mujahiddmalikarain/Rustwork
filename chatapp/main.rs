use std ::io:: {Errorkind,Read,Write};
use std ::net::Tcplistener;
use std::sync::mpsc;
use std::thread;

const local:&str="127.0.0.1:6000"';
const MSG_SIZE: usize =32;

