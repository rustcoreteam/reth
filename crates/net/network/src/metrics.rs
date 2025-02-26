use metrics::Histogram;
use reth_eth_wire::DisconnectReason;
use reth_metrics::{
    metrics::{Counter, Gauge},
    Metrics,
};
use reth_primitives::TxType;

/// Scope for monitoring transactions sent from the manager to the tx manager
pub(crate) const NETWORK_POOL_TRANSACTIONS_SCOPE: &str = "network.pool.transactions";

/// Metrics for the entire network, handled by NetworkManager
#[derive(Metrics)]
#[metrics(scope = "network")]
pub struct NetworkMetrics {
    /// Number of currently connected peers
    pub(crate) connected_peers: Gauge,

    /// Number of currently backed off peers
    pub(crate) backed_off_peers: Gauge,

    /// Number of peers known to the node
    pub(crate) tracked_peers: Gauge,

    /// Cumulative number of failures of pending sessions
    pub(crate) pending_session_failures: Counter,

    /// Total number of sessions closed
    pub(crate) closed_sessions: Counter,

    /// Number of active incoming connections
    pub(crate) incoming_connections: Gauge,

    /// Number of active outgoing connections
    pub(crate) outgoing_connections: Gauge,

    /// Total Number of incoming connections handled
    pub(crate) total_incoming_connections: Counter,

    /// Total Number of outgoing connections established
    pub(crate) total_outgoing_connections: Counter,

    /// Number of invalid/malformed messages received from peers
    pub(crate) invalid_messages_received: Counter,

    /// Number of Eth Requests dropped due to channel being at full capacity
    pub(crate) total_dropped_eth_requests_at_full_capacity: Counter,

    /* ================ POLL DURATION ================ */

    /* -- Total poll duration of `NetworksManager` future -- */
    /// Duration in seconds of call to
    /// [`NetworkManager`](crate::NetworkManager)'s poll function.
    ///
    /// True duration of this call, should be sum of the accumulated durations of calling nested
    // items.
    pub(crate) duration_poll_network_manager: Gauge,

    /* -- Poll duration of items nested in `NetworkManager` future -- */
    /// Time spent streaming messages sent over the [`NetworkHandle`](crate::NetworkHandle), which
    /// can be cloned and shared via [`NetworkManager::handle`](crate::NetworkManager::handle), in
    /// one call to poll the [`NetworkManager`](crate::NetworkManager) future.
    ///
    /// Duration in seconds.
    // todo: find out how many components hold the network handle.
    pub(crate) duration_poll_network_handle: Gauge,
    /// Time spent polling [`Swarm`](crate::swarm::Swarm), in one call to poll the
    /// [`NetworkManager`](crate::NetworkManager) future.
    ///
    /// Duration in seconds.
    pub(crate) duration_poll_swarm: Gauge,
}

/// Metrics for SessionManager
#[derive(Metrics)]
#[metrics(scope = "network")]
pub struct SessionManagerMetrics {
    /// Number of dials that resulted in a peer being added to the peerset
    pub(crate) total_dial_successes: Counter,
}

/// Metrics for the [`TransactionsManager`](crate::transactions::TransactionsManager).
#[derive(Metrics)]
#[metrics(scope = "network")]
pub struct TransactionsManagerMetrics {
    /* ================ BROADCAST ================ */
    /// Total number of propagated transactions
    pub(crate) propagated_transactions: Counter,
    /// Total number of reported bad transactions
    pub(crate) reported_bad_transactions: Counter,

    /* -- Freq txns already marked as seen by peer -- */
    /// Total number of messages from a peer, announcing transactions that have already been
    /// marked as seen by that peer.
    pub(crate) messages_with_hashes_already_seen_by_peer: Counter,
    /// Total number of messages from a peer, with transaction that have already been marked as
    /// seen by that peer.
    pub(crate) messages_with_transactions_already_seen_by_peer: Counter,
    /// Total number of occurrences, of a peer announcing a transaction that has already been
    /// marked as seen by that peer.
    pub(crate) occurrences_hash_already_seen_by_peer: Counter,
    /// Total number of times a transaction is seen from a peer, that has already been marked as
    /// seen by that peer.
    pub(crate) occurrences_of_transaction_already_seen_by_peer: Counter,

    /* -- Freq txns already in pool -- */
    /// Total number of times a hash is announced that is already in the local pool.
    pub(crate) occurrences_hashes_already_in_pool: Counter,
    /// Total number of times a transaction is sent that is already in the local pool.
    pub(crate) occurrences_transactions_already_in_pool: Counter,

    /* ================ POOL IMPORTS ================ */
    /// Number of transactions about to be imported into the pool.
    pub(crate) pending_pool_imports: Gauge,
    /// Total number of bad imports.
    pub(crate) bad_imports: Counter,
    /// Number of inflight requests at which the
    /// [`TransactionPool`](reth_transaction_pool::TransactionPool) is considered to be at
    /// capacity. Note, this is not a limit to the number of inflight requests, but a health
    /// measure.
    pub(crate) capacity_pending_pool_imports: Counter,
    /// Currently active outgoing [`GetPooledTransactions`](reth_eth_wire::GetPooledTransactions)
    /// requests.
    /* ================ TX FETCHER ================ */
    pub(crate) inflight_transaction_requests: Gauge,
    /// Number of inflight requests at which the
    /// [`TransactionFetcher`](crate::transactions::TransactionFetcher) is considered to be at
    /// capacity. Note, this is not a limit to the number of inflight requests, but a health
    /// measure.
    pub(crate) capacity_inflight_requests: Counter,
    /// Hashes in currently active outgoing
    /// [`GetPooledTransactions`](reth_eth_wire::GetPooledTransactions) requests.
    pub(crate) hashes_inflight_transaction_requests: Gauge,
    /// How often we failed to send a request to the peer because the channel was full.
    pub(crate) egress_peer_channel_full: Counter,
    /// Total number of hashes pending fetch.
    pub(crate) hashes_pending_fetch: Gauge,

    /* ================ POLL DURATION ================ */

    /* -- Total poll duration of `TransactionsManager` future -- */
    /// Duration in seconds of call to
    /// [`TransactionsManager`](crate::transactions::TransactionsManager)'s poll function.
    ///
    /// Updating metrics could take time, so the true duration of this call could
    /// be longer than the sum of the accumulated durations of polling nested items.
    pub(crate) duration_poll_tx_manager: Gauge,

    /* -- Poll duration of items nested in `TransactionsManager` future -- */
    /// Accumulated time spent streaming session updates and updating peers accordingly, in
    /// one call to poll the [`TransactionsManager`](crate::transactions::TransactionsManager)
    /// future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_poll_network_events: Gauge,
    /// Accumulated time spent flushing the queue of batched pending pool imports into pool, in
    /// one call to poll the [`TransactionsManager`](crate::transactions::TransactionsManager)
    /// future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_poll_pending_pool_imports: Gauge,
    /// Accumulated time spent streaming transaction and announcement broadcast, queueing for
    /// pool import or requesting respectively, in one call to poll the
    /// [`TransactionsManager`](crate::transactions::TransactionsManager) future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_poll_transaction_events: Gauge,
    /// Accumulated time spent streaming fetch events, queueing for pool import on successful
    /// fetch, in one call to poll the
    /// [`TransactionsManager`](crate::transactions::TransactionsManager) future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_poll_fetch_events: Gauge,
    /// Accumulated time spent streaming and propagating transactions that were successfully
    /// imported into the pool, in one call to poll the
    /// [`TransactionsManager`](crate::transactions::TransactionsManager) future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_poll_imported_transactions: Gauge,
    /// Accumulated time spent assembling and sending requests for hashes fetching pending, in
    /// one call to poll the [`TransactionsManager`](crate::transactions::TransactionsManager)
    /// future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_fetch_pending_hashes: Gauge,
    /// Accumulated time spent streaming commands and propagating, fetching and serving
    /// transactions accordingly, in one call to poll the
    /// [`TransactionsManager`](crate::transactions::TransactionsManager) future.
    ///
    /// Duration in seconds.
    pub(crate) acc_duration_poll_commands: Gauge,
}

/// Measures the duration of executing the given code block. The duration is added to the given
/// accumulator value passed as a mutable reference.
#[macro_export]
macro_rules! duration_metered_exec {
    ($code:block, $acc:ident) => {
        let start = Instant::now();

        $code;

        *$acc += start.elapsed();
    };
}

/// Metrics for Disconnection types
///
/// These are just counters, and ideally we would implement these metrics on a peer-by-peer basis,
/// in that we do not double-count peers for `TooManyPeers` if we make an outgoing connection and
/// get disconnected twice
#[derive(Metrics)]
#[metrics(scope = "network")]
pub struct DisconnectMetrics {
    /// Number of peer disconnects due to DisconnectRequested (0x00)
    pub(crate) disconnect_requested: Counter,

    /// Number of peer disconnects due to TcpSubsystemError (0x01)
    pub(crate) tcp_subsystem_error: Counter,

    /// Number of peer disconnects due to ProtocolBreach (0x02)
    pub(crate) protocol_breach: Counter,

    /// Number of peer disconnects due to UselessPeer (0x03)
    pub(crate) useless_peer: Counter,

    /// Number of peer disconnects due to TooManyPeers (0x04)
    pub(crate) too_many_peers: Counter,

    /// Number of peer disconnects due to AlreadyConnected (0x05)
    pub(crate) already_connected: Counter,

    /// Number of peer disconnects due to IncompatibleP2PProtocolVersion (0x06)
    pub(crate) incompatible: Counter,

    /// Number of peer disconnects due to NullNodeIdentity (0x07)
    pub(crate) null_node_identity: Counter,

    /// Number of peer disconnects due to ClientQuitting (0x08)
    pub(crate) client_quitting: Counter,

    /// Number of peer disconnects due to UnexpectedHandshakeIdentity (0x09)
    pub(crate) unexpected_identity: Counter,

    /// Number of peer disconnects due to ConnectedToSelf (0x0a)
    pub(crate) connected_to_self: Counter,

    /// Number of peer disconnects due to PingTimeout (0x0b)
    pub(crate) ping_timeout: Counter,

    /// Number of peer disconnects due to SubprotocolSpecific (0x10)
    pub(crate) subprotocol_specific: Counter,
}

impl DisconnectMetrics {
    /// Increments the proper counter for the given disconnect reason
    pub(crate) fn increment(&self, reason: DisconnectReason) {
        match reason {
            DisconnectReason::DisconnectRequested => self.disconnect_requested.increment(1),
            DisconnectReason::TcpSubsystemError => self.tcp_subsystem_error.increment(1),
            DisconnectReason::ProtocolBreach => self.protocol_breach.increment(1),
            DisconnectReason::UselessPeer => self.useless_peer.increment(1),
            DisconnectReason::TooManyPeers => self.too_many_peers.increment(1),
            DisconnectReason::AlreadyConnected => self.already_connected.increment(1),
            DisconnectReason::IncompatibleP2PProtocolVersion => self.incompatible.increment(1),
            DisconnectReason::NullNodeIdentity => self.null_node_identity.increment(1),
            DisconnectReason::ClientQuitting => self.client_quitting.increment(1),
            DisconnectReason::UnexpectedHandshakeIdentity => self.unexpected_identity.increment(1),
            DisconnectReason::ConnectedToSelf => self.connected_to_self.increment(1),
            DisconnectReason::PingTimeout => self.ping_timeout.increment(1),
            DisconnectReason::SubprotocolSpecific => self.subprotocol_specific.increment(1),
        }
    }
}

/// Metrics for the EthRequestHandler
#[derive(Metrics)]
#[metrics(scope = "network")]
pub struct EthRequestHandlerMetrics {
    /// Number of received headers requests
    pub(crate) received_headers_requests: Counter,

    /// Number of received bodies requests
    pub(crate) received_bodies_requests: Counter,
}

/// Eth67 announcement metrics, track entries by TxType
#[derive(Metrics)]
#[metrics(scope = "network.transaction_fetcher")]
pub struct AnnouncedTxTypesMetrics {
    /// Histogram for tracking frequency of legacy transaction type
    pub(crate) legacy: Histogram,

    /// Histogram for tracking frequency of EIP-2930 transaction type
    pub(crate) eip2930: Histogram,

    /// Histogram for tracking frequency of EIP-1559 transaction type
    pub(crate) eip1559: Histogram,

    /// Histogram for tracking frequency of EIP-4844 transaction type
    pub(crate) eip4844: Histogram,
}

#[derive(Debug, Default)]
pub struct TxTypesCounter {
    pub(crate) legacy: usize,
    pub(crate) eip2930: usize,
    pub(crate) eip1559: usize,
    pub(crate) eip4844: usize,
}

impl TxTypesCounter {
    pub(crate) fn increase_by_tx_type(&mut self, tx_type: TxType) {
        match tx_type {
            TxType::Legacy => {
                self.legacy += 1;
            }
            TxType::EIP2930 => {
                self.eip2930 += 1;
            }
            TxType::EIP1559 => {
                self.eip1559 += 1;
            }
            TxType::EIP4844 => {
                self.eip4844 += 1;
            }
            #[cfg(feature = "optimism")]
            TxType::DEPOSIT => {}
        }
    }
}

impl AnnouncedTxTypesMetrics {
    /// Update metrics during announcement validation, by examining each announcement entry based on
    /// TxType
    pub(crate) fn update_eth68_announcement_metrics(&self, tx_types_counter: TxTypesCounter) {
        self.legacy.record(tx_types_counter.legacy as f64);
        self.eip2930.record(tx_types_counter.eip2930 as f64);
        self.eip1559.record(tx_types_counter.eip1559 as f64);
        self.eip4844.record(tx_types_counter.eip4844 as f64);
    }
}
