$address = "192.168.99.202"
$port = "23"
$command_map = @{
    		(1, 1) => b"//F00U01I01Z\r";
		(1, 2) => b"//F00U01I02Y\r";
		(1, 3) => b"//F00U01I03X\r";
		(1, 4) => b"//F00U01I04_\r";
		(1, 5) => b"//F00U01I05^\r";
		(1, 6) => b"//F00U01I06]\r";
		(1, 7) => b"//F00U01I07\\r";
		(1, 8) => b"//F00U01I08S\r";
		(2, 1) => b"//F00U02I01Y\r";
		(2, 2) => b"//F00U02I02Z\r";
		(2, 3) => b"//F00U02I03[\r",
		(2, 4) => b"//F00U02I04\\r",
		(2, 5) => b"//F00U02I05]\r",
		(2, 6) => b"//F00U02I06^\r",
		(2, 7) => b"//F00U02I07_\r",
		(2, 8) => b"//F00U02I08P\r",
		(3, 1) => b"//F00U03I01X\r",
		(3, 2) => b"//F00U03I02[\r",
		(3, 3) => b"//F00U03I03Z\r",
		(3, 4) => b"//F00U03I04]\r",
		(3, 5) => b"//F00U03I05\\r",
		(3, 6) => b"//F00U03I06_\r",
		(3, 7) => b"//F00U03I07^\r",
		(3, 8) => b"//F00U03I08Q\r",
		(4, 1) => b"//F00U04I01_\r",
		(4, 2) => b"//F00U04I02\\r",
		(4, 3) => b"//F00U04I03]\r",
		(4, 4) => b"//F00U04I04Z\r",
		(4, 5) => b"//F00U04I05[\r",
		(4, 6) => b"//F00U04I06X\r",
		(4, 7) => b"//F00U04I07Y\r",
		(4, 8) => b"//F00U04I08V\r",
		(5, 1) => b"//F00U05I01^\r",
		(5, 2) => b"//F00U05I02]\r",
		(5, 3) => b"//F00U05I03\\r",
		(5, 4) => b"//F00U05I04[\r",
		(5, 5) => b"//F00U05I05Z\r",
		(5, 6) => b"//F00U05I06Y\r",
		(5, 7) => b"//F00U05I07X\r",
		(5, 8) => b"//F00U05I08W\r",
		(6, 1) => b"//F00U06I01]\r",
		(6, 2) => b"//F00U06I02^\r",
		(6, 3) => b"//F00U06I03_\r",
		(6, 4) => b"//F00U06I04X\r",
		(6, 5) => b"//F00U06I05Y\r",
		(6, 6) => b"//F00U06I06Z\r",
		(6, 7) => b"//F00U06I07[\r",
		(6, 8) => b"//F00U06I08T\r",
		(7, 1) => b"//F00U07I01\\r",
		(7, 2) => b"//F00U07I02_\r",
		(7, 3) => b"//F00U07I03^\r",
		(7, 4) => b"//F00U07I04Y\r",
		(7, 5) => b"//F00U07I05X\r",
		(7, 6) => b"//F00U07I06[\r",
		(7, 7) => b"//F00U07I07Z\r",
		(7, 8) => b"//F00U07I08U\r",
		(8, 1) => b"//F00U08I01S\r",
		(8, 2) => b"//F00U08I02P\r",
		(8, 3) => b"//F00U08I03Q\r",
		(8, 4) => b"//F00U08I04V\r",
		(8, 5) => b"//F00U08I05W\r",
		(8, 6) => b"//F00U08I06T\r",
		(8, 7) => b"//F00U08I07U\r",
		(8, 8) => b"//F00U08I08Z\r"		
}

function tcp_entry {
	$tcp_handle = New-Object System.Net.Sockets.TcpClient($router_address, $port)
	$tcp_stream = $tcp_handle.GetStream()
	$reader = New-Object System.IO.StreamReader($tcp_stream)
	$writer = New-Object System.IO.StreamWriter($tcp_stream)
	$writer.AutoFlush = $true

	while ($tcp.Connected) {
		write-host([char] $reader.Read()) -NoNewline
		while(($reader.Peek() -ne -1) -or ($tcp_handle.Available)){        
		    write-host ([char] $reader.Read()) -NoNewline
		}

		if ($tcp_handle.Connected) {
			    Write-Host -NoNewline "_"
			    $command = Read-Host
	
			    if ($command -eq "escape") {
			        break
			    }
		
			    $writer.WriteLine($command) | Out-Null
			}
		}
	}

	$reader.Close()
	$writer.Close()
	$tcp_handle.Close()
}

function 

function create_command_map {
	
}

function create_coordinate_id {
	param (
		[Parameter()] [Int] $x,
		[Parameter()] [Int] $y
	)
		
	return [System.Tuple]::Create($x, $y)
}
