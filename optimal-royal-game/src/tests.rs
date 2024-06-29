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
fn get_next_states_test() {}

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
