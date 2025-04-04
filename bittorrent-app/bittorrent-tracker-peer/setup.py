from setuptools import setup, find_packages

setup(
    name='bittorrent-tracker-peer',
    version='0.1.0',
    author='Amith Athreya H',
    author_email='aathreyah@acm.org',
    description='A BitTorrent-like hybrif P2P implementation ',
    packages=find_packages(),
    install_requires=[
    ],
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: OS Independent',
    ],
    python_requires='>=3.6',
)