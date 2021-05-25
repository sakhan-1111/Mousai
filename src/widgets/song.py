# song.py
#
# Copyright 2021 SeaDve
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.


from gi.repository import GObject


class Song(GObject.GObject):

    title = GObject.Property(type=str)
    artist = GObject.Property(type=str)
    song_link = GObject.Property(type=str)
    song_src = GObject.Property(type=str)

    def __init__(self, title, artist, song_link, song_src=''):
        super().__init__()

        self.title = title
        self.artist = artist
        self.song_link = song_link
        self.song_src = song_src

    def __iter__(self):
        yield 'title', self.title
        yield 'artist', self.artist
        yield 'song_link', self.song_link
        yield 'song_src', self.song_src