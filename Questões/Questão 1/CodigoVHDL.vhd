library ieee;
use ieee.std_logic_1164.all;

entity questao is
	port(
		A, B : in std_logic;
		X    : out std_logic
	);
end questao;

architecture arq_questao of questao is
begin
	X <= not(not(B) nand (A or B))
end arq_questao;