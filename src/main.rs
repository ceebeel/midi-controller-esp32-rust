use anyhow::{Context, Result};
use midir::{MidiOutput, MidiOutputPort};
// use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

fn main() -> Result<()> {
    let ports = serialport::available_ports().context("No serial port found")?;

    println!("Available serial ports:");
    for (i, port) in ports.iter().enumerate() {
        println!("{}: {}", i, port.port_name);
    }

    print!("Choose a port (number): ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let port_index: usize = input.trim().parse()?;

    let port_name = &ports[port_index].port_name;

    let mut serial_port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .with_context(|| format!("Failed to open port {}", port_name))?;

    let midi_out = MidiOutput::new("My ESP32-MIDI bridge")?;
    let port_out = choose_midi_port(&midi_out)?;
    let mut conn_out = midi_out.connect(&port_out, "esp32-midi")?;

    println!("ESP32-MIDI bridge started. Press Ctrl-C to quit.");

    let mut midi_buffer = Vec::new();

    loop {
        let mut serial_buf = [0u8; 1];
        match serial_port.read(&mut serial_buf) {
            Ok(1) => {
                midi_buffer.push(serial_buf[0]);
                if midi_buffer.len() == 3 {
                    // println!("MIDI message received: {:?}", midi_buffer);
                    conn_out.send(&midi_buffer)?;
                    midi_buffer.clear();
                }
            }
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("Error reading from serial port: {:?}", e),
        }
    }
}

fn choose_midi_port(midi_out: &MidiOutput) -> Result<MidiOutputPort> {
    let out_ports = midi_out.ports();
    let out_port = match out_ports.len() {
        0 => return Err(anyhow::anyhow!("No MIDI output port found")),
        1 => {
            println!(
                "Choosing the default MIDI port: {}",
                midi_out.port_name(&out_ports[0])?
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable MIDI output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p)?);
            }
            print!("Please choose a port (number): ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or_else(|| anyhow::anyhow!("Invalid port"))?
        }
    };
    Ok(out_port.clone())
}
