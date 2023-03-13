# intensidad_de_la_actividad_fisica_y_el_deporte
Uso de AI para crear las ecuaciones lineales de cintas de mediciones de latidos por minuto, con el mínimo de error.
Using AI to create the linear equations of beat-per-minute tape measurements, with minimal error.

### main.rs
archivo fuente en RUST 
source file in RUST

### resultados.txt
descripción del análisis de datos.csv ajustados al tipo de información que guardo en datos.csv.
el ejemplo que se tiene en este apartado es de adultos mayores en las bases de datos **AdultoMayor_n_fc de Kaggle https://www.kaggle.com/enriquebuenda/datasets?scroll=true **

description of the analysis of data.csv adjusted to the type of information that I save in data.csv.
The example in this section is of older adults in the databases **AdultoMayor_n_fc de Kaggle https://www.kaggle.com/enriquebuenda/datasets?scroll=true **


#### &nbsp;&nbsp;&nbsp;&nbsp;Opcion 1
&nbsp;&nbsp;Considera las bandas desde 0 a FC máxima
&nbsp;&nbsp;Consider the bands from 0 to maximum HR
#### &nbsp;&nbsp;&nbsp;&nbsp;Opción 2
&nbsp;&nbsp;Considera las bandas desde FC mínima a FC máxima
&nbsp;&nbsp;Consider the bands from minimum HR to maximum HR

![image](https://user-images.githubusercontent.com/44904277/224584689-9b99b247-f444-4baa-97b6-3f8b59420c50.png)


### datos.csv
son dos columnas edad y frecuencia cardiaca, en archivo CSV
(en este caso tiene información de adultos mayores realizando actividades diarias, enfermos y sanos, hombres y mujeres)
es necesario que tenga mediciones de frecuencia cardiaca en reposo o basal y las frecuencias cardiacas altas registradas en su actividad física,
en todo el rango de edad del grupo, varias veces al día (entre más mejor) y por varias semanas (incluyendo sábados y domingos).
Por lo que las ecuaciones de resultados.txt estiman frecuencias cardiacas (FC) para lograr la intensidad descrita en la imagen inferior de ejemplo, de personas aultas mayores, enfermas o no, mujeres u hombres.

Pero si lo que tiene en sus datos.csv es de un grupo élite de atletas, las ecuaciones estimarán las intensidades correspondientes de ese grupo específico, considerando como para cualquier otro grupo medir: FC en reposo o Basal, actividad rutinaria (no importa si son diferentes medidas diarias) y FC maximas registradas por cualquier instrumento o medio, por varias semanas (entre mas mejor), **considerar que las FC y las edades solo se leeran si son números enteros**.

Por lo que este es el archivo que usted debe usar como base para sus conclusiones e interpretaciones, el nivel de exigencia también dependerá de usted.

La app ai_intendidad.exe se ajustará a sus necesidades.

there are two columns age and heart rate, in CSV file
(in this case, it has information on older adults doing daily activities, sick and healthy, men and women)
you need to have resting or basal heart rate measurements and high heart rates recorded in your physical activity,
throughout the age range of the group, several times a day (the more the better) and for several weeks (including Saturdays and Sundays).
Therefore, the equations of results.txt estimate heart rates (HR) to achieve the intensity described in the lower example image, of older people, sick or not, women or men.

But if what you have in your data.csv is from an elite group of athletes, the equations will estimate the corresponding intensities of that specific group, considering as for any other group to measure: HR at rest or Basal, routine activity (it does not matter if they are different daily measurements) and maximum FC recorded by any instrument or means, for several weeks (the more the better), **consider that FC and ages will only be read if they are whole numbers**.

So this is the file that you should use as the basis for your conclusions and interpretations, the level of demand will also depend on you.

The ai_intendidad.exe app will adjust to your needs.

## AI_Intensidad.exe
Poner todos los archivos en una sola carpeta de Windows: AI_intensidad.exe, datos.csv (que son las mediciones de FC de varias actividades dependiendo la aplicación, en este caso adultos mayores, aunque se puede usar para cada grupo especifico), los **resultados.txt** se obtendrán de usar una ventana de MS DOS o Windows PowerShell estando en la carpeta de archivos mencionada, con el comando:  c:\carpeta_en_uso\ai_intensidad.exe > resultados.txt

El signo **>** redirige la salida de la app al archivo resultados.txt o con el nombre que desee.

Put all the files in a single Windows folder: AI_intensidad.exe, data.csv (which are the HR measurements of various activities depending on the application, in this case older adults, although it can be used for each specific group), the * *results.txt** will be obtained from using an MS DOS or Windows PowerShell window while in the mentioned file folder, with the command: c:carta_en_usoai_intensidad.exe > results.txt

The **>** sign redirects the output of the app to the file results.txt or whatever name you want.

![image](https://user-images.githubusercontent.com/44904277/224556163-130b5e64-f458-4eec-b44c-e061c71cc606.png)

