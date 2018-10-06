<#  
.SYNOPSIS 
    Sends an MHC preset UDP datagram to a server
.DESCRIPTION 
    Sends an MHC preset UDP datagram to a server
.NOTES 
    File Name  : Send-UDPDatagram 
    Author     : Rafał Michalski - royaltm75@gmail.com
    Requires   : PowerShell V2 CTP3
.EXAMPLE 
#> 

###
#  Start
##

param (
    [int]$Preset = 99,
    [string]$IP = "127.0.0.1",
    [int]$Port = 5000,
    [int]$SrcPort = 50001
)

$Address = [System.Net.IPAddress]::Parse($IP)

# Create IP Endpoint
$LocalIP = [System.Net.IPAddress]::Any
$Local = New-Object System.Net.IPEndPoint $LocalIP, $SrcPort
$End = New-Object System.Net.IPEndPoint $address, $port

# Create Socket
$Saf   = [System.Net.Sockets.AddressFamily]::InterNetwork
$Stype = [System.Net.Sockets.SocketType]::Dgram
$Ptype = [System.Net.Sockets.ProtocolType]::UDP
$Sock  = New-Object System.Net.Sockets.Socket $Saf, $Stype, $Ptype
$Sock.TTL = 36

# Connect to socket
$Sock.Bind($Local)
$Sock.Connect($End)

# Create MHC data
$Data = @(0x15,0x06,0x01,0x00,$Preset,0x66)

# Send data
$Sent = $Sock.Send($Data)
"{0} bytes sent to: {1}:{2}" -f $Sent,$IP,$Port

$How = [System.Net.Sockets.SocketShutdown]::Both
$Sock.Shutdown($How);
$Sock.Close();
# End of Script
