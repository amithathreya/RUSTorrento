# BitTorrent Tracker and Peer

This project implements a BitTorrent-like tracker and peer system in Python. It allows peers to connect, register with the tracker, and share data with each other.

## Project Structure

```
bittorrent-tracker-peer
├── tracker
│   ├── __init__.py
│   ├── tracker.py
│   └── utils.py
├── peer
│   ├── __init__.py
│   ├── peer.py
│   └── utils.py
├── tests
│   ├── test_tracker.py
│   ├── test_peer.py
│   └── __init__.py
├── requirements.txt
├── setup.py
└── README.md
```

## Installation

To install the required dependencies, run:

```
pip install -r requirements.txt
```

## Usage

1. **Starting the Tracker**: 
   To start the tracker, run the following command:
   ```
   python -m tracker.tracker
   ```

2. **Connecting a Peer**:
   To connect a peer to the tracker, use the following command:
   ```
   python -m peer.peer
   ```

3. **Interacting with Peers**:
   Once connected, peers can download and upload data to each other.

## Testing

To run the tests for the tracker and peer modules, execute:

```
pytest tests/
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bugs.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.