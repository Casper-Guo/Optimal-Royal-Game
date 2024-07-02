use std::hash::Hash;

use crate::board::{get_next_states, set_endgame_status, verify_state};
use crate::HashSet;

#[test]
fn verify_state_test() {
    assert!(verify_state(16140971502043660629).is_err());
    assert!(verify_state(14168336155652696576).is_err());

    assert!(verify_state(15132694669066115664).is_ok());
    assert!(verify_state(17762196930349236240).is_ok());
    assert!(verify_state(9368894599886536704).is_ok());
    assert!(verify_state(13980580618850795520).is_ok());
}

#[test]
fn get_next_states_test() {
    let mut expected = HashSet::from([2630106752246366736, 3206602517123317776]);
    let mut actual = HashSet::from_iter(get_next_states(3206567470190051856, 4, 1));
    assert_eq!(expected, actual);

    expected = HashSet::from([
        3134509876152123924,
        3206567470173274640,
        3783028153773998336,
        3206633440887964176,
    ]);
    actual = HashSet::from_iter(get_next_states(3206567470190051856, 2, 0));
    assert_eq!(expected, actual);

    expected = HashSet::from([3098481525792983105, 3098516128197132353]);
    actual = HashSet::from_iter(get_next_states(3098481493580728385, 2, 1));
    assert_eq!(expected, actual);

    expected = HashSet::from([36028797018963968]);
    actual = HashSet::from_iter(get_next_states(36028797035741184, 2, 0));
    assert_eq!(expected, actual);

    expected = HashSet::from([16777216]);
    actual = HashSet::from_iter(get_next_states(36028797035741184, 1, 1));
    assert_eq!(expected, actual);

    // regression testing
    expected = HashSet::from([17726172533524283392]);
    actual = HashSet::from_iter(get_next_states(17726168135477755968, 4, 0));
    assert_eq!(expected, actual);

    expected = HashSet::from([297527846477267024, 297265063197249552]);
    actual = HashSet::from_iter(get_next_states(297247471011139664, 4, 0));
    assert_eq!(expected, actual);
}

#[test]
fn set_endgame_status_test() {
    // black win
    assert_eq!(set_endgame_status(145522563031760896), 9368894599886536704);
    assert_eq!(set_endgame_status(9368894599886536704), 9368894599886536704);

    // white win
    assert_eq!(set_endgame_status(1155173304428920832), 5766859322856308736);
    assert_eq!(set_endgame_status(5766859322856308736), 5766859322856308736);

    // in progress
    assert_eq!(set_endgame_status(145522563568631808), 13980580618850795520);
    assert_eq!(
        set_endgame_status(13980580618850795520),
        13980580618850795520
    );
    assert_eq!(
        set_endgame_status(1227230898466848768),
        15062288953749012480
    );
    assert_eq!(set_endgame_status(1688849866555392), 13836746905148719104);
}
