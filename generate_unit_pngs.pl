#!/usr/bin/perl -w
use strict;
# The base of the freeciv source tree
my $fc_base = $ARGV[0] or die;
my $units_fn = "$fc_base/data/amplio2/units.png";
-r $units_fn or die "$!: '$units_fn'";
my $ofn = "/tmp/QQQ";
my $cmd = "convert -crop 64x48+X+Y $units_fn $ofn.png"; 
for(my $x = 0; $x < 20; $x++){
    for(my $y = 0; $y < 3; $y++){
	# convert -crop 64x48+1+1
	my $_x = $x * 65 + 1;
	my $_y = $y * 49 + 1;
	my $_cmd = $cmd;
	$_cmd =~ s/X/$_x/;
	$_cmd =~ s/Y/$_y/;
	my $q = sprintf("%02dx%02d", $x, $y);
	$_cmd =~ s/QQQ/$q/;
	print `$_cmd`;
    }
}


