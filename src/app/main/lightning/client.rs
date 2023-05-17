use ldk::ln::channelmanager::{ChannelManager, ChannelDetails};
use ldk::ln::channelmanager::HTLCSource::OutboundHTLC;
use ldk::ln::channelmanager::HTLCFailReason::ReasonNone;
use ldk::ln::channelmanager::PendingHTLCStatus::{OfferedHTLC, ReceivedHTLC};
use ldk::ln::chan_utils::{HTLCOutputInCommitment, ChannelPublicKeys};
use ldk::ln::msgs::UpdateAddHTLC;
use ldk::ln::msgs::CommitmentUpdate;
use ldk::ln::chan_utils::HTLCType::Offered;
use ldk::ln::chan_utils::HTLCType::Accepted;
use ldk::ln::msgs::UpdateFulfillHTLC;
use ldk::ln::msgs::UpdateFailHTLC;
use ldk::ln::msgs::UpdateFailMalformedHTLC;
use ldk::ln::msgs::ChannelMessageHandler;
use ldk::prelude::*;

fn create_htlc(channel_manager: &ChannelManager<ChannelMessageHandler>) {
    let payment_hash = PaymentHash([0x11; 32]);
    let payment_secret = PaymentSecret([0x22; 32]);
    let cltv_expiry = CltvExpiry(144);
    let htlc_value_msat = 1000;

    let local_channel_pubkeys = ChannelPublicKeys {
        funding_pubkey: PublicKey([0x01; 33]),
        revocation_basepoint: PublicKey([0x02; 33]),
        payment_point: PublicKey([0x03; 33]),
        delayed_payment_basepoint: PublicKey([0x04; 33]),
        htlc_basepoint: PublicKey([0x05; 33]),
    };
    let remote_channel_pubkeys = ChannelPublicKeys {
        funding_pubkey: PublicKey([0x06; 33]),
        revocation_basepoint: PublicKey([0x07; 33]),
        payment_point: PublicKey([0x08; 33]),
        delayed_payment_basepoint: PublicKey([0x09; 33]),
        htlc_basepoint: PublicKey([0x0a; 33]),
    };
    let htlc = HTLCOutputInCommitment {
        offered: OfferedHTLC {
            payment_hash,
            cltv_expiry,
            amount_msat: htlc_value_msat,
            onion_routing_packet: OnionPacket::from_bytes(&[0x00; 1254]).unwrap(),
            pubkey: remote_channel_pubkeys.payment_point,
        },
        accepted: None,
        kind: Offered,
        commitment_index: 0,
    };
    let htlc_index = 0;

    let (prev_commitment, _, _) = channel_manager.get_channel_details(&OutPoint { txid: Txid([0x00; 32]), index: 0 }).unwrap();
    let mut commitment_tx = prev_commitment.commitment_tx.clone();

    let channel_id = ChannelDetails {
        channel_id: [0x01; 32],
        remote_network_id: [0x02; 33],
        remote_channel_pubkeys: remote_channel_pubkeys.clone(),
        local_channel_pubkeys: local_channel_pubkeys.clone(),
