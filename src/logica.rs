

pub fn num_letras(line :String)->String{
    let str3: &str = &line.trim();
    let mut res:String=String::from("");
    if(str3.len()==3){
        let mut  num=str3.parse::<usize>().unwrap();
  
      res= nom_centenas(num);
      }else if(str3.len()>=4){
        
        let mut chars :Vec<&str> = str3.split('.').collect();
       let millones:Vec<String>= vec!["MILLÓN".to_string(), "BILLÓN".to_string(),
         "TRILLÓN".to_string() ,"CUATRILLÓN".to_string() ,"QUINTILLÓN".to_string() ,"SEXTILLÓN".to_string() ,
        "SEPTILLÓN".to_string() ,"OCTILLÓN".to_string() ,"NONILLÓN".to_string() ,"DECILLÓN".to_string(),
        "UNDECILLÓN".to_string(),"DUODECILLÓN".to_string(),"TREDECILLÓN".to_string(),"CUATORDECILLÓN".to_string(),
        "QUINDECILLÓN".to_string(),"SEXDECILLÓN".to_string(),"SEPTENDECILLÓN".to_string(),"OCTODECILLÓN".to_string(),
        "NOVENDECILLÓN".to_string(),"VIGINTILLÓN".to_string()];
        
        let mut contador=chars.len()-7;
     
        for i in 0..chars.len(){
           
                if(i==chars.len()){
                    let mut  num=chars[i].parse::<usize>().unwrap();
                  res=  nom_centenas(num).to_string();
                }else if( i%2!=0){
                    let mut  num=chars[i].parse::<usize>().unwrap();
                  res=  nom_centenas(num);
                    
                          
                   if(contador<=millones.len()){
                  res=  millones[contador].to_string();
                    if(contador !=0){
                        contador =contador-1;
                    }
                         
                    } 
                    
                }else{
                let mut  num=chars[i].parse::<usize>().unwrap();
               res= nom_centenas(num)+" mil ";
    
            }

        }
    }else if(str3.len()<3){
        let mut  num=str3.parse::<usize>().unwrap();
      
         
         res= nom_decenas(num);
      }
      return res;
}



fn nom_decenas(num:usize)-> String{
   
    let unidades:Vec<String>= vec!["cero".to_string(), "uno".to_string(),
     "dos".to_string() ,"tres".to_string() ,"cuatro".to_string() ,"cinco".to_string() ,
    "seis".to_string() ,"siete".to_string() ,"ocho".to_string() ,"nueve".to_string(),
    "diez".to_string()];
    let especiales:Vec<String>=vec!["once".to_string(), "doce".to_string(),"trece".to_string()
    ,"catorce".to_string(), "quince".to_string(),"diezciseis".to_string(), "diecisiete".to_string(),
     "dieciocho".to_string(), "diecinueve".to_string()];
    let decenas:Vec<String>=vec!["veinte".to_string(), "treinta".to_string(),"cuarenta".to_string(),
    "cincuenta".to_string(), "sesenta".to_string(),"setenta".to_string(), "ochenta".to_string(), "noventa".to_string()];
   let mut res_dec:String=String::from("");
    
    if(num>=0 && num<11){
            res_dec=  unidades[num].to_string();         
    }else if(num<20){
        res_dec=  especiales[num-11].to_string();        
     } else if(num<100){
           let mut unid = num % 10;
            let mut dec = num/10;
            if(unid == 0){
                res_dec=  decenas[dec-2].to_string();                
            }else {
                
            res_dec = decenas[dec-2].to_string()+ "y"+ &unidades[unid];
                
                
            }
            
        }
        return res_dec;
    }
    



    fn nom_centenas(num:usize)->String{
        let mut res_cen:String;
        let centenas:Vec<String>=vec!["docientos".to_string(), "trecientos ".to_string(),"cuatrocientos".to_string(),
        "quinientos ".to_string(),"seiscientos ".to_string(), "setecientos ".to_string(), "ochocientos ".to_string(),
        "novecientos ".to_string()];
        let mut decena= num % 100;
        let mut centena= num - decena;

        if(centena/100 == 0){

         return  nom_decenas(decena);

        }else{
        if(num==100){
          return  "cien".to_string();
        }else{
            
            if(centena==100){
               res_cen= "ciento ".to_string()+ &nom_decenas(decena).to_string();
            }else{
              
                let mut obtner_centena=(centena/100)-2;
              res_cen=  centenas[obtner_centena].to_string();
            if(decena > 0){
             let  res_uni=  nom_decenas(decena);
             res_cen=res_cen + &res_uni;
            }    
        }
        return  res_cen;
    }
            
           
        }
    }
   fn nom_mil(str3:String){
    let chars:Vec<char> = str3.chars().collect();
    let mut centena=String::from("");
    let mut centena2=String::from("");
    for i in 0..str3.len()/2{
          centena.push(chars[i]);
        
    }
    for i in (3..str3.len()){
         centena2.push(chars[i]);
        
    }
        
   let mut num_centena=centena.parse::<usize>().unwrap();
    let mut num_centena2=centena2.parse::<usize>().unwrap();
         
    nom_centenas(num_centena); format!("mil ");
    nom_centenas(num_centena2);
   
}