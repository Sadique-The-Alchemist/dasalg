

SELECT 
pm.pcode as 'Customer code',
pm.pname as  'Customer name',
tdt.tdate as 'Transaction date',
 tdt.vchno as 'Voucher No',
 tdt.ttype,
 tde.tamount as 'Amount', 
scl.cwgt as 'Weight',
 scl.grate as 'Rate'
FROM transdet tde 
INNER JOIN transdata tdt ON tde.slno=tdt.slno
INNER JOIN scollect scl ON scl.serial=tde.slno
INNER JOIN personm pm ON pm.pcode= tde.acbcode



SELECT 
pm.pcode as 'Customer code',
pm.pname as  'Customer name',
tdt.tdate as 'Transaction date',
 tdt.vchno as 'Voucher No',
 tdt.ttype,
 tde.tamount as 'Amount', 
scl.cwgt as 'Weight',
 scl.grate as 'Rate'
FROM transdet tde 
INNER JOIN transdata tdt ON tde.slno=tdt.slno
INNER JOIN scollect scl ON scl.serial=tde.slno
INNER JOIN personm pm ON pm.pcode= tde.acbcode
INNER JOIN
(SELECT 
pm.pcode as pcode,
pm.pname,
SUM(CASE WHEN tdt.ttype ='SLC' THEN tde.tamount WHEN tdt.ttype='PMT' THEN -(tde.tamount) ELSE 0 END ) as 'Amount', 
SUM(CASE WHEN tdt.ttype ='SLC' THEN scl.cwgt WHEN tdt.ttype='PMT' THEN -scl.cwgt ELSE 0 END ) as 'Weight',
AVG(scl.grate) as 'Rate'
FROM transdet tde 
INNER JOIN transdata tdt ON tde.slno=tdt.slno
INNER JOIN scollect scl ON scl.serial=tde.slno
INNER JOIN personm pm ON pm.pcode= tde.acbcode
GROUP BY pm.pcode,pm.pname
HAVING  SUM(CASE WHEN tdt.ttype ='SLC' THEN tde.tamount WHEN tdt.ttype='PMT' THEN -(tde.tamount) ELSE 0 END ) !=0 
) as net_amount ON pm.pcode = net_amount.pcode