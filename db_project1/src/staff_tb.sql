--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name character(50) NOT NULL,
    dno integer NOT NULL,
    staff_sal integer NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	Mustapha Ali                                      	3	175000	32	08063285831
107	Alokwe                                            	7	380000	48	07090082812
102	Mankinde Mary                                     	2	450000	55	09023688832
120	Adeleke Jane                                      	4	200000	38	07061045862
117	Suleman Ajayi                                     	3	800000	50	07030089842
104	kuti lawal                                        	1	750000	35	09145689842
122	Osahon Mark                                       	6	320000	44	70999899989
108	Josiah Joshua                                     	1	120000	30	08099899989
97	Dankande Aminat                                   	5	550000	40	08099899989
101	Alade                                             	2	250000	33	07099899989
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

