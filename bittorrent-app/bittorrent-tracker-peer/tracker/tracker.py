class Tracker:
    def __init__(self):
        self.peers = set()

    def add_peer(self, peer_id):
        self.peers.add(peer_id)

    def remove_peer(self, peer_id):
        self.peers.discard(peer_id)

    def get_peers(self):
        return list(self.peers)