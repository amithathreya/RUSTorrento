import unittest
from tracker.tracker import Tracker

class TestTracker(unittest.TestCase):

    def setUp(self):
        self.tracker = Tracker()

    def test_add_peer(self):
        self.tracker.add_peer('peer1')
        self.assertIn('peer1', self.tracker.get_peers())

    def test_remove_peer(self):
        self.tracker.add_peer('peer2')
        self.tracker.remove_peer('peer2')
        self.assertNotIn('peer2', self.tracker.get_peers())

    def test_get_peers(self):
        self.tracker.add_peer('peer3')
        self.tracker.add_peer('peer4')
        peers = self.tracker.get_peers()
        self.assertEqual(len(peers), 2)
        self.assertIn('peer3', peers)
        self.assertIn('peer4', peers)

if __name__ == '__main__':
    unittest.main()