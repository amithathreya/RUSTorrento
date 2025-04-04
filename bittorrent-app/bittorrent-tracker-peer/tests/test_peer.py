import unittest
from peer.peer import Peer

class TestPeer(unittest.TestCase):

    def setUp(self):
        self.peer = Peer()

    def test_connect(self):
        # Test the connect method
        result = self.peer.connect('127.0.0.1', 6881)
        self.assertTrue(result)

    def test_download(self):
        # Test the download method
        result = self.peer.download('some_file')
        self.assertIsNotNone(result)

    def test_upload(self):
        # Test the upload method
        result = self.peer.upload('some_file')
        self.assertTrue(result)

if __name__ == '__main__':
    unittest.main()