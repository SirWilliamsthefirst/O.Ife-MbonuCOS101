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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_manageride integer NOT NULL,
    dno integer NOT NULL,
    dname character(50) NOT NULL,
    dlocation character(50) NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character(50) NOT NULL,
    pduration character varying NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

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
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_manageride, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration                                    	Ikeja                                             	44
101	2	Account                                           	Egbeda                                            	11
100	3	packaging                                         	Ajah                                              	44
120	4	Research                                          	VI                                                	33
97	5	Account                                           	Magodo                                            	22
122	6	Operations                                        	Mile                                              	44
107	7	packaging                                         	ketu                                              	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A                                                 	9 Months	102
22	B                                                 	14 Months	97
33	C                                                 	16 Months	120
44	D                                                 	25 Months	108
55	E                                                 	9 Months	107
\.


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
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_manageride);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

