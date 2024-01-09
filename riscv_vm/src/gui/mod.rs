pub mod modal;
mod vm_screen;

use std::fs;

use elf_load::Elf;
use modal::Modal;

use iced::{
    alignment::{Horizontal, Vertical},
    executor, theme,
    widget::{button, column, container, pick_list, row, scrollable, text, text_input},
    window, Application, Color, Command, Length, Settings, Theme,
};

use crate::{
    args::{Args, GraphicsMode},
    decode::instruction::Instruction,
    devices::{vga_text_mode::VgaTextMode, DeviceInitError},
    memory::{
        address::Address,
        registers::{IntRegister, Registers},
        MB,
    },
    vmstate::VMState,
};

use self::vm_screen::VmScreen;

pub fn run(flags: Args) -> Result<(), iced::Error> {
    EmulatorGui::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        id: Some("riscv_vm".into()),
        flags,
        ..Settings::default()
    })
}

struct EmulatorGui {
    vm: VMState<{ 4 * MB }>,
    step: bool,
    screen: VmScreen,
}

#[derive(Debug, Clone)]
enum Message {
    // CreateVM,
    // EditVM,
    // SaveVM,
    // StartAddDevice,
    // DeviceAdded(usize),
    // CancelAddDevice,
    // DeviceAddrChange(String),
    // LoadImage,
    None,
}

impl Application for EmulatorGui {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = Args;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let bytes = fs::read(flags.image).unwrap();
        let elf = Elf::from_bytes(bytes).unwrap();

        let mut vmstate = VMState::<{ 4 * MB }>::new(flags.hart_count as u64);
        vmstate.load_elf_kernel(&elf).unwrap();

        match flags.graphics_mode {
            Some(GraphicsMode::VgaText) => {
                if let Some(a) = flags.graphics_address {
                    vmstate.add_async_device::<VgaTextMode>(a.into()).unwrap();
                } else {
                    vmstate
                        .add_async_device::<VgaTextMode>(0xB8000u64.into())
                        .unwrap();
                }
            }
            None => {}
        }

        if flags.enable_breakpoints {
            vmstate.enable_breakpoints();
        }

        (
            Self {
                vm: vmstate,
                step: flags.step,
                screen: VmScreen::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RiscV Emulator".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            m => println!("{:?}", m),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let mut content = column![container(self.screen.view().map(|_| Message::None))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)];

        if self.step {
            content = content.push(
                container(row![
                    button("Step").padding(10),
                    container(text(format!(
                        "{:#?}",
                        Instruction::ADD {
                            rd: IntRegister::X5,
                            rs1: IntRegister::X7,
                            rs2: IntRegister::X10
                        }
                    )))
                    .padding(10)
                ])
                .width(Length::Fill)
                // .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            );
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
