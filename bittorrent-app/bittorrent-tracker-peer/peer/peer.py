from socket import *
import json
class Peer:
    def __init__(self, peer_id, address,metadata_file):
        self.peer_id = peer_id
        self.address = address
        self.connected_peers = []
        self.metadata = self.load_metadata(metadata_file)


    def load_metadata(self, metadata_file):
        try:
                with open(metadata_file, 'r') as file:
                    return json.load(file)
        except FileNotFoundError:
            print(f"Metadata file not found: {metadata_file}")
            return {}
        except json.JSONDecodeError:
            print(f"Error decoding JSON from metadata file: {metadata_file}")
            return {}

    def connect(self, peer_address):
        try:
            peer_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            peer_sock.connect(peer_address)
            self.connected_peers.append([peer_address,peer_sock])
            print(f"Connected to {peer_address}")
        except Exception as e :
            print(f"Failed to connect to {peer_address}: {str(e)}")

    def disconnect(self, peer_address):
        for i,([addr, sock]) in enumerate(self.connected_peers):
            if addr == peer_address:
                sock.close()
                del self.connected_peers[i]
                print(f"Disconnected from {peer_address}")
                return
        print(f"No connection found to {peer_address}")



    def download(self, piece_index):
        for peer_address,peer_sock in self.connected_peers:
            try:
                req_message = f"REQUEST PIECE{piece_index}"
                peer_sock.send(req_message.encode())
                response = peer_sock.recv(1024).decode()
                if self.verify_piece(response, piece_index):
                    print(f"Piece {piece_index} downloaded successfully from {peer_address}")
                    return response
                else:
                    print(f"Piece {piece_index} verification failed from {peer_address}")
            except Exception as e:  
                print(f"Error downloading {piece_index} from {peer_address}: {str(e)}")
            print(f"Failed to download {piece_index} from {peer_address}")
            return None




    def upload(self, piece_index):
        for peer_address,peer_sock in self.connected_peers:
            try:
                request = peer_sock.recv(1024).decode()
                if request.startswith("REQUEST PIECE"):
                    req_piece_index = int(request.split()[-1])
                    if req_piece_index == piece_index:
                        piece_data = self.get_piece(piece_index)
                        peer_sock.sendall(piece_data)
                        print(f"Piece {piece_index} uploaded successfully to {peer_address}")
                    else:
                        print(f"Requested pirce {req_piece_index} does not match {piece_index}")
                else:
                    print(f"Invalid request received from {peer_address}")
            except Exception as e:
                print(f"Error uploading the requested piece {piece_index}to {peer_address} : " str(e))




    def __str__(self):
        return f"Peer {self.peer_id} at {self.address}"