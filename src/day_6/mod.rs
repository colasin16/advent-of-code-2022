use crate::helpers::{
    advent_headings::{print_day, print_part},
    file_system::read_file_lines,
};

const START_OF_PACKET_MARKER_LENGTH: usize = 4;
const START_OF_MESSAGE_MARKER_LENGTH: usize = 14;

pub fn execute(input: &str) {
    print_day(6);

    if let Ok(lines) = read_file_lines(input) {
        for line in lines {
            if let Ok(stream) = line {
                let start_of_packet_marker =
                    find_stream_marker_position(START_OF_PACKET_MARKER_LENGTH, stream.clone());

                if let Some(position) = start_of_packet_marker {
                    print_part(
                        1,
                        format!(
                            "The start-of-packet marker position is {}",
                            (position + 1).to_string()
                        ),
                    );
                }

                let start_of_message_marker =
                    find_stream_marker_position(START_OF_MESSAGE_MARKER_LENGTH, stream);

                if let Some(position) = start_of_message_marker {
                    print_part(
                        2,
                        format!(
                            "The start-of-message marker position is {}",
                            (position + 1).to_string()
                        ),
                    );
                }
            }
        }
    }
}

fn find_stream_marker_position(marker_length: usize, stream: String) -> Option<usize> {
    stream
        .clone()
        .chars()
        .enumerate()
        .position(|(index, current)| {
            if index >= marker_length - 1 {
                let marker = stream.get((index - (marker_length - 1))..index);

                match marker {
                    Some(m) => {
                        let repeated_character = m.chars().enumerate().find_map(|(i, curr)| {
                            m.chars()
                                .enumerate()
                                .skip(i + 1)
                                .find(|(_, other)| curr == *other)
                        });

                        repeated_character.is_none() && !m.contains(current)
                    }
                    None => false,
                }
            } else {
                false
            }
        })
}
