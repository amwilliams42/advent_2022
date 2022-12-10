use std::collections::HashSet;
use std::hash::Hash;
use std::io::{BufReader, Read};

pub fn start_of_packet<R: Read>(io: R) -> (usize,usize) {
    let mut bufread = BufReader::new(io);
    let mut buffer = Vec::new();
    bufread.read_to_end(&mut buffer).unwrap();

    let pack_start_windows = buffer.windows(4);
    let message_start_windows = buffer.windows(14);

    let mut packet_start: usize = 0;
    let mut message_start: usize = 0;



    for (i,w) in pack_start_windows.enumerate() {
        if is_all_unique(w){
            packet_start = i+4;
            break
        }
    }
    for (i,w) in message_start_windows.enumerate() {
        if is_all_unique(w){
            message_start = i+14;
            break
        }
    }
    (packet_start, message_start)
}

fn is_all_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash
{
    let mut unique = HashSet::new();
    iter.into_iter().all(|x| unique.insert(x))
}

#[cfg(test)]
mod tests {
    use crate::days::day6::start_of_packet;

    #[test]
    fn test_packet() {
        let res1 = start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes());
        let res2 = start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes());
        let res3 = start_of_packet("nppdvjthqldpwncqszvftbrmjlhg".as_bytes());
        let res4 = start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes());
        let res5 = start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes());

        assert_eq!(res1, (7, 19));
        assert_eq!(res2, (5, 23));
        assert_eq!(res3, (6, 23));
        assert_eq!(res4, (10, 29));
        assert_eq!(res5, (11, 26));

    }
}
