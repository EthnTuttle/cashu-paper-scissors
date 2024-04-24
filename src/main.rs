fn main() {
    // rock paper scissors clone asynchronously using BDHKE and cashu protocol (sort of).
    // Player1, Player2 want to play a game so talk to GameServer.
    // Player1 and 2 are ecash users, and GameServer is a cashu mint or DVM
    // The players will play 3 rounds of cashu-paper-scissors.
    // The players send a nostr note to GameServer indicating they would like to play.
    // the event of that note is then used for a new public keyset from the GameServer.
    // This event ID would be the "unit" in NUT-02
    // The keysey has one pubkey for each round of play.
    // Players create a nostr event indicating their 'action' for each round.
    // this is the 'x' value aka secret message, used in the BDHKE.
    // This is hashed to a point on the secp256k1 curve and then used in the standard blinding, signing and unblinding
    // as defined in NIP-00. We then have a state where each player has a signed nostr event as their input for a
    // blinded token generation, where the mint knows it signed something, but not the details. 
    // (this is different from cashu since validation of 'game logic' aka bitcoin/LN transaction would happen after signing, maybe)
    // Once all things have been signed, players can now publish their 'action" note, and reference it in a note 
    // with the unblinding signature. If you're looking at NUT-00, this is x and C. These are the things one user shares
    // with another to exchange ecash. Because the mint can take x and C and validate that the signature is valid.
    // The GameServer 'mint' now becomes a game oracle by validating the 'action' notes and the unblinded signature.
    // It can then publish the results as a nostr event for others to reference. I think the noteduel DLC stuff
    // could come into play here but haven't read up on that yet so haven't the slightest clue.
}
