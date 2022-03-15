#!/bin/env perl


use strict;
use warnings;
use File::Basename;
use Imager;
use Image::ExifTool;
use Data::Dumper;


my $MAX_PIXEL = 320;
my $IMAGEDIR = '/project/imgr/images/';


my $exiftool = new Image::ExifTool;
my %data = ();

for my $file (@ARGV) {
    my @s = stat $file;
    my @t = localtime($s[9]);
    # たぶん使わないけど一応残しておく
    my $mtime = sprintf("%4d%02d%02d%02d%02d%02d", $t[5] + 1900, $t[4] + 1, $t[3], $t[2], $t[1], $t[0]);

    my $exifinfo = $exiftool->ImageInfo($file);
    #print Dumper($exifinfo);

    my $timestr = '';
    if (defined($exifinfo->{DateTimeOriginal})) {
	$timestr = $exifinfo->{DateTimeOriginal};
    }
    elsif (defined($exifinfo->{DateCreate})) {
	$timestr = $exifinfo->{DateCreate};
    }
    elsif (defined($exifinfo->{FileModifyDate})) {
	$timestr = $exifinfo->{FileModifyDate};
    }

    $timestr =~ s/\+09:00//;

    die Dumper($exifinfo) unless $timestr;
    die Dumper($exifinfo) if ($timestr !~ /^\d\d\d\d:\d\d:\d\d \d\d:\d\d:\d\d$/);

    my $dir = $timestr;
    $dir =~ s/ .*//;
    $dir =~ s/://g;

    my $hmd = $timestr;
    $hmd =~ s/.* //;
    $hmd =~ s/://g;

    my $filename = basename $file;

#    mkdir $IMAGEDIR . $dir;
    printf("%s -> %s/%s_%s regist.\n", $file, $dir, $hmd, $filename);
}


#my $img = Imager->new(file=>$file) or die Imager->errstr . ':' . $file;
#my $thumb = $img->scale(xpixels =>$MAX_PIXEL, ypixels => $MAX_PIXEL, type => 'min');
#$thumb->filter( type => 'autolevels' );
#$thumb->write( file => sprintf '%s%02d.jpg', );
