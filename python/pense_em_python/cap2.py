#Exercicio 2.2

#1
def get_volume(raio):
    pi = 3.14
    volume = (4 * pi * raio**3)/3

    return print(volume)

get_volume(5)

#2
def calc_preco_livros(numero_exemplares):
    preco_unitario = 24.95
    preco_unitario_desconto = preco_unitario - preco_unitario * (40/100)
    transporte_primeiro_exempltar = 3
    transporte_demais_exemplares = 0.75

    custo_total = preco_unitario_desconto * numero_exemplares + transporte_primeiro_exempltar + (numero_exemplares - 1) * transporte_demais_exemplares

    return print(custo_total)

calc_preco_livros(60)

#3
def convert_horas(minutos, segundos):
    horas = (minutos/60) + (segundos/3600)

    return horas

def retorno_caminhada(hora_inicio, minuto_inicio):
    passo_lento = convert_horas(8, 15)
    passo_rapido = convert_horas(7, 12)

    total_caminhada_minutos = (1*passo_lento + 3*passo_rapido + 1*passo_lento)*60
    tempo_retorno = hora_inicio + (minuto_inicio + total_caminhada_minutos)//60 + ((minuto_inicio + total_caminhada_minutos)%60)/100

    return print(tempo_retorno)

retorno_caminhada(6, 52) 