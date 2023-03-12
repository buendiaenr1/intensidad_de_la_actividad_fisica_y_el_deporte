# intensidad_de_la_actividad_fisica_y_el_deporte
Uso de AI para crear las ecuaciones lineales de cintas de mediciones de latidos por minuto, con el mínimo de error.

### main.rs
archivo fuente en RUST 

### resultados.txt
descripción del análisis de datos.csv

#### Opcion 1
Considera las bandas desde 0 a FC maáxima
#### Opción 2
Considera las bandas desde FC mínima a FC máxima

### datos.csv
son dos columnas edad y frecuencia cardiaca, en archivo CSV
(en este caso tiene información de adultos mayores realizando actividades diarias, enfermos y sanos, hombres y mujeres)
es necesario que tenga mediciones de frecuencia cardiaca en reposo o basal y las frecuencias cardiacas altas registradas en su actividad física,
en todo el rango de edad del grupo, varias veces al día (entre más mejor) y por varias semanas (incluyendo sábados y domingos).
Por lo que las ecuaciones de resultados.txt estiman frecuencias cardiacas (FC) para lograr la intensidad descrita en la imagen inferior de ejemplo, de personas aultas mayores, enfermas o no, mujeres u hombres.

Pero si lo que tiene en sus datos.csv es de un grupo élite de atletas, las ecuaciones estimarán las intensidades correspondientes de ese grupo específico, considerando como para cualquier otro grupo medir: FC en reposo o Basal, actividad rutinaria (no importa si son diferentes medidas diarias) y FC maximas registradas por cualquier instrumento o medio, por varias semanas (entre mas mejor), **considerar que las FC y las edades solo se leeran si son números enteros**.


## AI_Intensidad.exe
Poner todos los archivos en una sola carpeta de Windows: AI_intensidad.exe, datos.csv (que son las mediciones de FC de varias actividades dependiendo la aplicación, en este caso adultos mayores, aunque se puede usar para cada grupo especifico), los **resultados.txt** se obtendrán de usar una ventana de MS DOS o Windows PowerShell estando en la carpeta de archivos mencionada, con el comando:  c:\carpeta_en_uso\ai_intensidad.exe > resultados.txt

El signo **>** redirige la salida de la app al archivo resultados.txt o con el nombre que desee.


![image](https://user-images.githubusercontent.com/44904277/224556163-130b5e64-f458-4eec-b44c-e061c71cc606.png)

