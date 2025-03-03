// Code generated automatically. DO NOT EDIT.

use crate::types::{AssetType, Link};
use crate::blockchain::Blockchain;
use crate::asset::Asset;

/// Bitcoin Blockchain Implementation
pub struct BitcoinBlockchain;

const LOGO_SVG: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" width=\"128\" height=\"128\">
<path d=\"M0 0 C1.30904297 0.54333984 1.30904297 0.54333984 2.64453125 1.09765625 C9.22693834 4.08046745 14.51154139 8.360177 20 13 C23.32114838 15.77544582 23.32114838 15.77544582 27 18 C27.495 19.485 27.495 19.485 28 21 C28.598125 21.5775 29.19625 22.155 29.8125 22.75 C33.22337455 26.25832811 35.82398292 29.93179931 36.25 34.9375 C36.1675 35.618125 36.085 36.29875 36 37 C36.66 37 37.32 37 38 37 C39.6801581 52.91461381 40.86867068 69.56152096 36 85 C35.85175781 85.67289062 35.70351562 86.34578125 35.55078125 87.0390625 C34.8746386 89.44632213 33.83236096 91.4371156 32.625 93.625 C32.20476562 94.40101562 31.78453125 95.17703125 31.3515625 95.9765625 C30 98 30 98 27 100 C26.505 101.485 26.505 101.485 26 103 C24.6818334 103.69617457 23.34555075 104.35834947 22 105 C20.59375018 106.2147372 19.22087172 107.46869039 17.875 108.75 C2.49193242 122.6249237 -14.2052491 124.80164926 -34.15771484 124.17919922 C-47.93633732 123.31049033 -59.60691807 115.49968879 -70 107 C-72.29386681 105.24382598 -74.59379224 103.60413851 -77 102 C-77.33 101.01 -77.66 100.02 -78 99 C-78.8971875 98.13375 -78.8971875 98.13375 -79.8125 97.25 C-84.33475103 92.5985418 -86.27678634 88.43515023 -86.78881836 82.00537109 C-86.97185789 79.71596036 -86.97185789 79.71596036 -88.00390625 76.4765625 C-89.38006629 71.11883674 -89.34059986 65.81661482 -89.3125 60.3125 C-89.31048584 59.25192383 -89.30847168 58.19134766 -89.30639648 57.09863281 C-89.20699058 49.32473408 -88.34577446 42.43841436 -86 35 C-85.77763672 33.99066406 -85.77763672 33.99066406 -85.55078125 32.9609375 C-84.8746386 30.55367787 -83.83236096 28.5628844 -82.625 26.375 C-82.20476562 25.59898438 -81.78453125 24.82296875 -81.3515625 24.0234375 C-80 22 -80 22 -77 20 C-76.67 19.01 -76.34 18.02 -76 17 C-74.6818334 16.30382543 -73.34555075 15.64165053 -72 15 C-70.59375018 13.7852628 -69.22087172 12.53130961 -67.875 11.25 C-50.12471628 -4.76005983 -22.11542037 -9.84701199 0 0 Z M-85 33 C-84 35 -84 35 -84 35 Z M34 33 C35 35 35 35 35 35 Z M-85 85 C-84 87 -84 87 -84 87 Z M34 85 C35 87 35 87 35 87 Z \" fill=\"#F89E33\" transform=\"translate(89,4)\"/>
<path d=\"M0 0 C2.31 0.33 4.62 0.66 7 1 C6 9.57142857 6 9.57142857 5 13 C6.98 13 8.96 13 11 13 C11.144375 11.906875 11.28875 10.81375 11.4375 9.6875 C12 6 12 6 13 3 C15.31 3.33 17.62 3.66 20 4 C19.67 7.96 19.34 11.92 19 16 C20.423125 16.7425 21.84625 17.485 23.3125 18.25 C26.76564327 20.05163997 29.03438047 21.58668609 31 25 C31.75035325 29.52418874 31.54646353 33.71581071 29.75 37.9375 C27.99338947 40.00779099 26.26737703 41.51447712 24 43 C24.804375 43.61875 25.60875 44.2375 26.4375 44.875 C29 47 29 47 30 49 C30.3962444 54.84460492 29.95926533 58.77196458 27 64 C23.54609516 67.65207971 20.28837896 69.89327176 15.15234375 70.1328125 C14.09144531 70.13023438 13.03054687 70.12765625 11.9375 70.125 C10.87402344 70.12757812 9.81054688 70.13015625 8.71484375 70.1328125 C6 70 6 70 4 69 C3.34 72.63 2.68 76.26 2 80 C-1.465 79.505 -1.465 79.505 -5 79 C-4.85904936 77.3745692 -4.71207913 75.74965949 -4.5625 74.125 C-4.48128906 73.22007813 -4.40007813 72.31515625 -4.31640625 71.3828125 C-4 69 -4 69 -3 67 C-4.98 66.67 -6.96 66.34 -9 66 C-9.66 69.63 -10.32 73.26 -11 77 C-13.31 76.67 -15.62 76.34 -18 76 C-17.01 70.06 -17.01 70.06 -16 64 C-22.435 62.515 -22.435 62.515 -29 61 C-28 55 -28 55 -27 53 C-24.36 53 -21.72 53 -19 53 C-17.84294165 48.85721727 -16.7014185 44.71030298 -15.5625 40.5625 C-15.23701172 39.39783203 -14.91152344 38.23316406 -14.57617188 37.03320312 C-12.83468577 30.65972307 -11.50053248 24.59151933 -11 18 C-13.31 17.01 -15.62 16.02 -18 15 C-17.67 12.69 -17.34 10.38 -17 8 C-11.76119922 8.55145271 -7.02649303 9.42922093 -2 11 C-1.855625 9.9275 -1.71125 8.855 -1.5625 7.75 C-1.16250558 5.08337051 -0.72408076 2.58600271 0 0 Z \" fill=\"#FEFCF9\" transform=\"translate(63,24)\"/>
<path d=\"M0 0 C12.9872913 2.18165961 12.9872913 2.18165961 17.9375 6.8125 C19.47297705 9.97377627 19.12032976 11.70903134 18 15 C14.04262601 18.12992307 11.30457185 18.27128806 6.375 17.75 C5.24835938 17.64171875 4.12171875 17.5334375 2.9609375 17.421875 C0 17 0 17 -3 16 C-2.68806536 14.27073126 -2.37542638 12.54158956 -2.0625 10.8125 C-1.88847656 9.84957031 -1.71445312 8.88664062 -1.53515625 7.89453125 C-1.0476265 5.25760028 -0.53328895 2.6280095 0 0 Z \" fill=\"#F89F34\" transform=\"translate(60,66)\"/>
<path d=\"M0 0 C10.66131621 1.97431782 10.66131621 1.97431782 14 5 C15.125 8.4375 15.125 8.4375 15 12 C12.92815345 14.60651663 11.38971954 15.77909891 8.05859375 16.30859375 C3.95597137 16.44093641 0.71711266 15.85855633 -3 14 C-2.48594831 9.03083364 -1.76615118 4.70973649 0 0 Z \" fill=\"#F8A33C\" transform=\"translate(66,45)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C2.3709375 1.556875 1.741875 2.11375 1.09375 2.6875 C-1.62051457 5.10744672 -4.31242265 7.55046521 -7 10 C-7.73734375 10.65742188 -8.4746875 11.31484375 -9.234375 11.9921875 C-14.57258504 16.97865502 -17.82203402 22.45146405 -21 29 C-22.32 29 -23.64 29 -25 29 C-24.34172278 25.79638418 -23.20286791 23.23428792 -21.625 20.375 C-20.99464844 19.21097656 -20.99464844 19.21097656 -20.3515625 18.0234375 C-19 16 -19 16 -16 14 C-15.67 13.01 -15.34 12.02 -15 11 C-13.68090483 10.30558646 -12.34446284 9.64392694 -11 9 C-9.42452931 7.68531541 -7.88403785 6.32793455 -6.375 4.9375 C-2.19444444 1.09722222 -2.19444444 1.09722222 0 0 Z \" fill=\"#FAAA3E\" transform=\"translate(28,10)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C-2.27349815 6.92143649 -8.16877757 11.02420872 -15 15 C-16.03833984 15.60521484 -16.03833984 15.60521484 -17.09765625 16.22265625 C-24.9000442 20.43626617 -33.15489555 22.50066629 -42 22 C-42 21.34 -42 20.68 -42 20 C-41.34515625 19.98582031 -40.6903125 19.97164062 -40.015625 19.95703125 C-24.23509997 19.1323428 -11.17558709 10.63570366 0 0 Z \" fill=\"#FA9920\" transform=\"translate(110,106)\"/>
<path d=\"M0 0 C12.57336404 0.41911213 21.71439124 7.05010272 31 15 C34.32052659 17.7760465 34.32052659 17.7760465 38 20 C38.495 21.485 38.495 21.485 39 23 C39.66 23.33 40.32 23.66 41 24 C40.67 24.66 40.34 25.32 40 26 C39.01 26 38.02 26 37 26 C35.23724122 24.18662145 33.62176528 22.36956075 32 20.4375 C23.06650542 10.3213696 12.43249234 5.58039192 0 1 C0 0.67 0 0.34 0 0 Z \" fill=\"#FCAB46\" transform=\"translate(78,2)\"/>
<path d=\"M0 0 C3.01606147 1.34998385 5.03493339 2.87390474 7.375 5.1875 C17.72021137 14.39819934 30.64792 17.84010471 44 20 C44 20.66 44 21.32 44 22 C29.36207269 23.14807273 15.95704575 15.47484357 4.875 6.61328125 C3.15404602 5.13254377 1.50177451 3.70357546 0 2 C0 1.34 0 0.68 0 0 Z \" fill=\"#FB9A20\" transform=\"translate(16,106)\"/>
<path d=\"M0 0 C0.33 0.99 0.66 1.98 1 3 C-0.36328125 5.6640625 -0.36328125 5.6640625 -2.3125 8.625 C-2.94800781 9.60726562 -3.58351563 10.58953125 -4.23828125 11.6015625 C-6 14 -6 14 -8 15 C-8.33 15.99 -8.66 16.98 -9 18 C-10.32 17.34 -11.64 16.68 -13 16 C-12.50628906 15.30132812 -12.01257813 14.60265625 -11.50390625 13.8828125 C-10.86324219 12.97273438 -10.22257812 12.06265625 -9.5625 11.125 C-8.92441406 10.22007813 -8.28632812 9.31515625 -7.62890625 8.3828125 C-6.00481862 6.12261419 -6.00481862 6.12261419 -5 4 C-4.34 4 -3.68 4 -3 4 C-3 3.34 -3 2.68 -3 2 C-2.01 2 -1.02 2 0 2 C0 1.34 0 0.68 0 0 Z \" fill=\"#FAA62A\" transform=\"translate(124,89)\"/>
<path d=\"M0 0 C-2.69005904 3.38235365 -5.99395322 4.26461328 -10 5.625 C-11.2375 6.05039063 -12.475 6.47578125 -13.75 6.9140625 C-15.83309261 7.61009585 -17.9164061 8.3054687 -20 9 C-19 6 -19 6 -17.36328125 4.90234375 C-6.17167655 -0.3630398 -6.17167655 -0.3630398 0 0 Z \" fill=\"#FCAD47\" transform=\"translate(52,1)\"/>
<path d=\"M0 0 C0.33 0.66 0.66 1.32 1 2 C2.32 1.67 3.64 1.34 5 1 C6.04449911 4.13349732 5.93423645 4.98968256 5 8 C3.02 8 1.04 8 -1 8 C-0.67 5.36 -0.34 2.72 0 0 Z \" fill=\"#F8A848\" transform=\"translate(69,29)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C4.62323754 4.75907641 5.23994929 7.92086213 5 12 C4.01 12.495 4.01 12.495 3 13 C0.89172945 9.83759418 0.66182535 8.49342531 0.375 4.8125 C0.30023438 3.91144531 0.22546875 3.01039062 0.1484375 2.08203125 C0.09945313 1.39496094 0.05046875 0.70789063 0 0 Z M2 7 C3 9 3 9 3 9 Z \" fill=\"#FAA92C\" transform=\"translate(2,82)\"/>
<path d=\"M0 0 C1.9375 0.3125 1.9375 0.3125 4 1 C4.33 1.99 4.66 2.98 5 4 C5.66 4.33 6.32 4.66 7 5 C6.67 5.66 6.34 6.32 6 7 C5.01 7 4.02 7 3 7 C0.64276511 4.46815512 0 3.52809907 0 0 Z \" fill=\"#F9AF3A\" transform=\"translate(112,21)\"/>
<path d=\"M0 0 C2.31 0.33 4.62 0.66 7 1 C6.4225 1.226875 5.845 1.45375 5.25 1.6875 C2.4336339 3.33038023 0.91922886 5.39533227 -1 8 C-0.67 5.36 -0.34 2.72 0 0 Z \" fill=\"#F79824\" transform=\"translate(67,93)\"/>
<path d=\"M0 0 C2.475 0.495 2.475 0.495 5 1 C4.625 3.9375 4.625 3.9375 4 7 C3.01 7.495 3.01 7.495 2 8 C1.34 5.36 0.68 2.72 0 0 Z \" fill=\"#FBA73B\" transform=\"translate(122,40)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C2.01 1 1.02 1 0 1 C0 0.67 0 0.34 0 0 Z \" fill=\"#FBA83C\" transform=\"translate(94,5)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(81,127)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(44,127)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(84,126)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(41,126)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(89,124)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(36,124)\"/>
<path d=\"\" fill=\"#FCC132\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FCC132\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FCC132\" transform=\"translate(0,0)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(89,3)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(36,3)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(84,1)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(41,1)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(81,0)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(44,0)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(87,125)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(39,125)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(95,121)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(31,121)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(97,120)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(29,120)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(100,118)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(26,118)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(104,115)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(22,115)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(104,12)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(22,12)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(100,9)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(26,9)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(97,7)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(29,7)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(31,6)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(87,2)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(39,2)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F95B5B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FFFF42\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FFFF42\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FFFF42\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
</svg>
";

impl Blockchain for BitcoinBlockchain {
    fn name(&self) -> &str {
        "Bitcoin"
    }

    fn description(&self) -> &str {
        "Bitcoin is a cryptocurrency and worldwide payment system. It is the first decentralized digital currency, as the system works without a central bank or single administrator.
"
    }

    fn website(&self) -> &str {
        "https://bitcoin.org"
    }

    fn explorer(&self) -> &str {
        "https://blockchain.info"
    }

    fn symbol(&self) -> &str {
        "BTC"
    }

    fn decimals(&self) -> u32 {
        8
    }

    fn links(&self) -> Vec<Link> {
        vec![
            Link { name: "github".to_string(), url: "https://github.com/bitcoin".to_string() },
            Link { name: "twitter".to_string(), url: "https://twitter.com/Bitcoin".to_string() },
            Link { name: "reddit".to_string(), url: "https://reddit.com/r/Bitcoin".to_string() },
            Link { name: "whitepaper".to_string(), url: "https://bitcoin.org/bitcoin.pdf".to_string() },
        ]
    }

    fn assets(&self) -> Vec<Box<dyn Asset>> {
        vec![
            Box::new(BitcoinBTCAsset),
        ]
    }

    fn asset(&self, id: &str) -> Option<Box<dyn Asset>> {
        match id {
            "bitcoin_btc" => Some(Box::new(BitcoinBTCAsset)),
            _ => None,
        }
    }

    fn logo(&self) -> &str {
            LOGO_SVG
    }
}


/// Bitcoin Asset Implementation
pub struct BitcoinBTCAsset;

const ICON_BITCOINBTCASSET: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" width=\"128\" height=\"128\">
<path d=\"M0 0 C1.30904297 0.54333984 1.30904297 0.54333984 2.64453125 1.09765625 C9.22693834 4.08046745 14.51154139 8.360177 20 13 C23.32114838 15.77544582 23.32114838 15.77544582 27 18 C27.495 19.485 27.495 19.485 28 21 C28.598125 21.5775 29.19625 22.155 29.8125 22.75 C33.22337455 26.25832811 35.82398292 29.93179931 36.25 34.9375 C36.1675 35.618125 36.085 36.29875 36 37 C36.66 37 37.32 37 38 37 C39.6801581 52.91461381 40.86867068 69.56152096 36 85 C35.85175781 85.67289062 35.70351562 86.34578125 35.55078125 87.0390625 C34.8746386 89.44632213 33.83236096 91.4371156 32.625 93.625 C32.20476562 94.40101562 31.78453125 95.17703125 31.3515625 95.9765625 C30 98 30 98 27 100 C26.505 101.485 26.505 101.485 26 103 C24.6818334 103.69617457 23.34555075 104.35834947 22 105 C20.59375018 106.2147372 19.22087172 107.46869039 17.875 108.75 C2.49193242 122.6249237 -14.2052491 124.80164926 -34.15771484 124.17919922 C-47.93633732 123.31049033 -59.60691807 115.49968879 -70 107 C-72.29386681 105.24382598 -74.59379224 103.60413851 -77 102 C-77.33 101.01 -77.66 100.02 -78 99 C-78.8971875 98.13375 -78.8971875 98.13375 -79.8125 97.25 C-84.33475103 92.5985418 -86.27678634 88.43515023 -86.78881836 82.00537109 C-86.97185789 79.71596036 -86.97185789 79.71596036 -88.00390625 76.4765625 C-89.38006629 71.11883674 -89.34059986 65.81661482 -89.3125 60.3125 C-89.31048584 59.25192383 -89.30847168 58.19134766 -89.30639648 57.09863281 C-89.20699058 49.32473408 -88.34577446 42.43841436 -86 35 C-85.77763672 33.99066406 -85.77763672 33.99066406 -85.55078125 32.9609375 C-84.8746386 30.55367787 -83.83236096 28.5628844 -82.625 26.375 C-82.20476562 25.59898438 -81.78453125 24.82296875 -81.3515625 24.0234375 C-80 22 -80 22 -77 20 C-76.67 19.01 -76.34 18.02 -76 17 C-74.6818334 16.30382543 -73.34555075 15.64165053 -72 15 C-70.59375018 13.7852628 -69.22087172 12.53130961 -67.875 11.25 C-50.12471628 -4.76005983 -22.11542037 -9.84701199 0 0 Z M-85 33 C-84 35 -84 35 -84 35 Z M34 33 C35 35 35 35 35 35 Z M-85 85 C-84 87 -84 87 -84 87 Z M34 85 C35 87 35 87 35 87 Z \" fill=\"#F89E33\" transform=\"translate(89,4)\"/>
<path d=\"M0 0 C2.31 0.33 4.62 0.66 7 1 C6 9.57142857 6 9.57142857 5 13 C6.98 13 8.96 13 11 13 C11.144375 11.906875 11.28875 10.81375 11.4375 9.6875 C12 6 12 6 13 3 C15.31 3.33 17.62 3.66 20 4 C19.67 7.96 19.34 11.92 19 16 C20.423125 16.7425 21.84625 17.485 23.3125 18.25 C26.76564327 20.05163997 29.03438047 21.58668609 31 25 C31.75035325 29.52418874 31.54646353 33.71581071 29.75 37.9375 C27.99338947 40.00779099 26.26737703 41.51447712 24 43 C24.804375 43.61875 25.60875 44.2375 26.4375 44.875 C29 47 29 47 30 49 C30.3962444 54.84460492 29.95926533 58.77196458 27 64 C23.54609516 67.65207971 20.28837896 69.89327176 15.15234375 70.1328125 C14.09144531 70.13023438 13.03054687 70.12765625 11.9375 70.125 C10.87402344 70.12757812 9.81054688 70.13015625 8.71484375 70.1328125 C6 70 6 70 4 69 C3.34 72.63 2.68 76.26 2 80 C-1.465 79.505 -1.465 79.505 -5 79 C-4.85904936 77.3745692 -4.71207913 75.74965949 -4.5625 74.125 C-4.48128906 73.22007813 -4.40007813 72.31515625 -4.31640625 71.3828125 C-4 69 -4 69 -3 67 C-4.98 66.67 -6.96 66.34 -9 66 C-9.66 69.63 -10.32 73.26 -11 77 C-13.31 76.67 -15.62 76.34 -18 76 C-17.01 70.06 -17.01 70.06 -16 64 C-22.435 62.515 -22.435 62.515 -29 61 C-28 55 -28 55 -27 53 C-24.36 53 -21.72 53 -19 53 C-17.84294165 48.85721727 -16.7014185 44.71030298 -15.5625 40.5625 C-15.23701172 39.39783203 -14.91152344 38.23316406 -14.57617188 37.03320312 C-12.83468577 30.65972307 -11.50053248 24.59151933 -11 18 C-13.31 17.01 -15.62 16.02 -18 15 C-17.67 12.69 -17.34 10.38 -17 8 C-11.76119922 8.55145271 -7.02649303 9.42922093 -2 11 C-1.855625 9.9275 -1.71125 8.855 -1.5625 7.75 C-1.16250558 5.08337051 -0.72408076 2.58600271 0 0 Z \" fill=\"#FEFCF9\" transform=\"translate(63,24)\"/>
<path d=\"M0 0 C12.9872913 2.18165961 12.9872913 2.18165961 17.9375 6.8125 C19.47297705 9.97377627 19.12032976 11.70903134 18 15 C14.04262601 18.12992307 11.30457185 18.27128806 6.375 17.75 C5.24835938 17.64171875 4.12171875 17.5334375 2.9609375 17.421875 C0 17 0 17 -3 16 C-2.68806536 14.27073126 -2.37542638 12.54158956 -2.0625 10.8125 C-1.88847656 9.84957031 -1.71445312 8.88664062 -1.53515625 7.89453125 C-1.0476265 5.25760028 -0.53328895 2.6280095 0 0 Z \" fill=\"#F89F34\" transform=\"translate(60,66)\"/>
<path d=\"M0 0 C10.66131621 1.97431782 10.66131621 1.97431782 14 5 C15.125 8.4375 15.125 8.4375 15 12 C12.92815345 14.60651663 11.38971954 15.77909891 8.05859375 16.30859375 C3.95597137 16.44093641 0.71711266 15.85855633 -3 14 C-2.48594831 9.03083364 -1.76615118 4.70973649 0 0 Z \" fill=\"#F8A33C\" transform=\"translate(66,45)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C2.3709375 1.556875 1.741875 2.11375 1.09375 2.6875 C-1.62051457 5.10744672 -4.31242265 7.55046521 -7 10 C-7.73734375 10.65742188 -8.4746875 11.31484375 -9.234375 11.9921875 C-14.57258504 16.97865502 -17.82203402 22.45146405 -21 29 C-22.32 29 -23.64 29 -25 29 C-24.34172278 25.79638418 -23.20286791 23.23428792 -21.625 20.375 C-20.99464844 19.21097656 -20.99464844 19.21097656 -20.3515625 18.0234375 C-19 16 -19 16 -16 14 C-15.67 13.01 -15.34 12.02 -15 11 C-13.68090483 10.30558646 -12.34446284 9.64392694 -11 9 C-9.42452931 7.68531541 -7.88403785 6.32793455 -6.375 4.9375 C-2.19444444 1.09722222 -2.19444444 1.09722222 0 0 Z \" fill=\"#FAAA3E\" transform=\"translate(28,10)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C-2.27349815 6.92143649 -8.16877757 11.02420872 -15 15 C-16.03833984 15.60521484 -16.03833984 15.60521484 -17.09765625 16.22265625 C-24.9000442 20.43626617 -33.15489555 22.50066629 -42 22 C-42 21.34 -42 20.68 -42 20 C-41.34515625 19.98582031 -40.6903125 19.97164062 -40.015625 19.95703125 C-24.23509997 19.1323428 -11.17558709 10.63570366 0 0 Z \" fill=\"#FA9920\" transform=\"translate(110,106)\"/>
<path d=\"M0 0 C12.57336404 0.41911213 21.71439124 7.05010272 31 15 C34.32052659 17.7760465 34.32052659 17.7760465 38 20 C38.495 21.485 38.495 21.485 39 23 C39.66 23.33 40.32 23.66 41 24 C40.67 24.66 40.34 25.32 40 26 C39.01 26 38.02 26 37 26 C35.23724122 24.18662145 33.62176528 22.36956075 32 20.4375 C23.06650542 10.3213696 12.43249234 5.58039192 0 1 C0 0.67 0 0.34 0 0 Z \" fill=\"#FCAB46\" transform=\"translate(78,2)\"/>
<path d=\"M0 0 C3.01606147 1.34998385 5.03493339 2.87390474 7.375 5.1875 C17.72021137 14.39819934 30.64792 17.84010471 44 20 C44 20.66 44 21.32 44 22 C29.36207269 23.14807273 15.95704575 15.47484357 4.875 6.61328125 C3.15404602 5.13254377 1.50177451 3.70357546 0 2 C0 1.34 0 0.68 0 0 Z \" fill=\"#FB9A20\" transform=\"translate(16,106)\"/>
<path d=\"M0 0 C0.33 0.99 0.66 1.98 1 3 C-0.36328125 5.6640625 -0.36328125 5.6640625 -2.3125 8.625 C-2.94800781 9.60726562 -3.58351563 10.58953125 -4.23828125 11.6015625 C-6 14 -6 14 -8 15 C-8.33 15.99 -8.66 16.98 -9 18 C-10.32 17.34 -11.64 16.68 -13 16 C-12.50628906 15.30132812 -12.01257813 14.60265625 -11.50390625 13.8828125 C-10.86324219 12.97273438 -10.22257812 12.06265625 -9.5625 11.125 C-8.92441406 10.22007813 -8.28632812 9.31515625 -7.62890625 8.3828125 C-6.00481862 6.12261419 -6.00481862 6.12261419 -5 4 C-4.34 4 -3.68 4 -3 4 C-3 3.34 -3 2.68 -3 2 C-2.01 2 -1.02 2 0 2 C0 1.34 0 0.68 0 0 Z \" fill=\"#FAA62A\" transform=\"translate(124,89)\"/>
<path d=\"M0 0 C-2.69005904 3.38235365 -5.99395322 4.26461328 -10 5.625 C-11.2375 6.05039063 -12.475 6.47578125 -13.75 6.9140625 C-15.83309261 7.61009585 -17.9164061 8.3054687 -20 9 C-19 6 -19 6 -17.36328125 4.90234375 C-6.17167655 -0.3630398 -6.17167655 -0.3630398 0 0 Z \" fill=\"#FCAD47\" transform=\"translate(52,1)\"/>
<path d=\"M0 0 C0.33 0.66 0.66 1.32 1 2 C2.32 1.67 3.64 1.34 5 1 C6.04449911 4.13349732 5.93423645 4.98968256 5 8 C3.02 8 1.04 8 -1 8 C-0.67 5.36 -0.34 2.72 0 0 Z \" fill=\"#F8A848\" transform=\"translate(69,29)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C4.62323754 4.75907641 5.23994929 7.92086213 5 12 C4.01 12.495 4.01 12.495 3 13 C0.89172945 9.83759418 0.66182535 8.49342531 0.375 4.8125 C0.30023438 3.91144531 0.22546875 3.01039062 0.1484375 2.08203125 C0.09945313 1.39496094 0.05046875 0.70789063 0 0 Z M2 7 C3 9 3 9 3 9 Z \" fill=\"#FAA92C\" transform=\"translate(2,82)\"/>
<path d=\"M0 0 C1.9375 0.3125 1.9375 0.3125 4 1 C4.33 1.99 4.66 2.98 5 4 C5.66 4.33 6.32 4.66 7 5 C6.67 5.66 6.34 6.32 6 7 C5.01 7 4.02 7 3 7 C0.64276511 4.46815512 0 3.52809907 0 0 Z \" fill=\"#F9AF3A\" transform=\"translate(112,21)\"/>
<path d=\"M0 0 C2.31 0.33 4.62 0.66 7 1 C6.4225 1.226875 5.845 1.45375 5.25 1.6875 C2.4336339 3.33038023 0.91922886 5.39533227 -1 8 C-0.67 5.36 -0.34 2.72 0 0 Z \" fill=\"#F79824\" transform=\"translate(67,93)\"/>
<path d=\"M0 0 C2.475 0.495 2.475 0.495 5 1 C4.625 3.9375 4.625 3.9375 4 7 C3.01 7.495 3.01 7.495 2 8 C1.34 5.36 0.68 2.72 0 0 Z \" fill=\"#FBA73B\" transform=\"translate(122,40)\"/>
<path d=\"M0 0 C0.99 0.33 1.98 0.66 3 1 C2.01 1 1.02 1 0 1 C0 0.67 0 0.34 0 0 Z \" fill=\"#FBA83C\" transform=\"translate(94,5)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(81,127)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(44,127)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(84,126)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(41,126)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(89,124)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(36,124)\"/>
<path d=\"\" fill=\"#FCC132\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FCC132\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F4B52B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FCC132\" transform=\"translate(0,0)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(89,3)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(36,3)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(84,1)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#FCC132\" transform=\"translate(41,1)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(81,0)\"/>
<path d=\"M0 0 C3 1 3 1 3 1 Z \" fill=\"#F4B52B\" transform=\"translate(44,0)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(87,125)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(39,125)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(95,121)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(31,121)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(97,120)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(29,120)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(100,118)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(26,118)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(104,115)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(22,115)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(104,12)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FA7F42\" transform=\"translate(22,12)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(100,9)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(26,9)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(97,7)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(29,7)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FDD136\" transform=\"translate(31,6)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(87,2)\"/>
<path d=\"M0 0 C2 1 2 1 2 1 Z \" fill=\"#FBA32A\" transform=\"translate(39,2)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#F95B5B\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FFFF42\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FFFF42\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FFFF42\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#E47D16\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
<path d=\"\" fill=\"#FBA32A\" transform=\"translate(0,0)\"/>
</svg>
";

impl Asset for BitcoinBTCAsset {
    fn id(&self) -> &str {
        "bitcoin_btc"
    }

    fn name(&self) -> &str {
        "Bitcoin"
    }

    fn symbol(&self) -> &str {
        "BTC"
    }

    fn asset_type(&self) -> AssetType {
        "coin"
    }

    fn asset_kind(&self) -> &str {
        "NATIVE"
    }

    fn description(&self) -> &str {
        "Bitcoin is a cryptocurrency and worldwide payment system. It is the first decentralized digital currency, as the system works without a central bank or single administrator.
"
    }

    fn website(&self) -> &str {
        "https://bitcoin.org"
    }

    fn explorer(&self) -> &str {
        "https://blockchain.info"
    }

    fn decimals(&self) -> u32 {
        8
    }

    fn status(&self) -> &str {
        "active"
    }

    fn icon(&self) -> &str {
            ICON_BITCOINBTCASSET
    }
}
