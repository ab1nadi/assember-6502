    use crate::assembler::insertable_num::InsertableNum;
    use crate::assembler::lexical_analyzer::*;
    use crate::assembler::gen_errors::*;
    use crate::assembler::Assembler;
    use std::num::Wrapping;

    // stack_math
    // does math between an operand stack and an operator stack
    // but only does it from a given index in the operator stack 
    // pops the left parenth off if the index given is one
    pub fn stack_math(operand_stack:&mut Vec<InsertableNum>, operator_stack:&mut Vec<Token>) -> Result<(),GeneralError> 
    {
        let mut index = 0;

        // get the next left parenth if it exists
        let possible = operator_stack
        .iter().rev()
        .position(|x| x.token_type== TokenType::LeftParenth);

        if let Some(i) = possible 
        {   
            index = operator_stack.len()-1-i;

            // remove the left parent too
            operator_stack.remove(index);
        }



        if operand_stack.len() == 1
        {
            return Ok(())
        }

        // do all the times and divide operations up the stack 
        let mut copy_index = index.clone();
        while copy_index != operator_stack.len() 
        {
            
            let operator = &operator_stack[copy_index];

            let operand_1_index = operand_stack.len() as i32 - (operator_stack.len() as i32 - copy_index as i32+1);


            let operand_1 = &operand_stack[operand_1_index as usize];
            let operand_2 = &operand_stack[operand_1_index as usize+1];


            if operator.token_type == TokenType::TIMES || operator.token_type == TokenType::DIVIDE
            {
                // replace the number on the stack
                let replaced_index = operand_1_index;

                operand_stack[replaced_index as usize] = do_operation(*operand_1, *operand_2, operator.clone())?;

                // remove the next position 
                operand_stack.remove(replaced_index as usize+1);

                operator_stack.remove(copy_index as usize);
            }
            else 
            {
                copy_index = copy_index+1;
            }
        }

         // do all the add and subtract operations up the stack 
         let mut copy_index = index.clone();
         while copy_index != operator_stack.len() 
         {
            let operator = &operator_stack[copy_index];

            let operand_1_index = operand_stack.len() as i32 - (operator_stack.len()as i32 - copy_index as i32+1);

            let operand_1 = &operand_stack[operand_1_index as usize];
            let operand_2 = &operand_stack[operand_1_index as usize+1];



            if operator.token_type == TokenType::PLUS || operator.token_type == TokenType::MINUS
            {
                // replace the number on the stack
                let replaced_index = operand_1_index;
                
                operand_stack[replaced_index as usize] = do_operation(*operand_1, *operand_2, operator.clone())?;

                // remove the next position 
                operand_stack.remove(replaced_index as usize+1);

                operator_stack.remove(copy_index as usize);
            }
            else 
            {
                copy_index = copy_index+1;
            }
         }
        Ok(())
    }



        // do_operation
    // takes the two operands 
    // and does the given operation
    pub fn do_operation(operand1:InsertableNum, operand2:InsertableNum, operator:Token)-> Result<InsertableNum, GeneralError>
    {
        match operator.token_type
        {
            TokenType::PLUS =>
            {
                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    return Ok(InsertableNum::TwoByte(operand1.unwrap_twobyte() + operand2.unwrap_twobyte()))
                }      
                else
                {
                    return Ok(InsertableNum::Byte((operand1.unwrap_byte() + operand2.unwrap_byte()) as u8))
                }          
            },
            TokenType::MINUS =>
            {
                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    let i =  Wrapping(operand1.unwrap_twobyte()) - Wrapping(operand2.unwrap_twobyte());
                    return Ok(InsertableNum::TwoByte(i.0))
                }      
                else
                {
                    let i = Wrapping(operand1.unwrap_byte()) - Wrapping(operand2.unwrap_byte());
                    return Ok(InsertableNum::Byte(i.0))
                }          
            },
            TokenType::TIMES =>
            {
                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    let i =  Wrapping(operand1.unwrap_twobyte()) * Wrapping(operand2.unwrap_twobyte());
                    return Ok(InsertableNum::TwoByte(i.0))
                }      
                else
                {
                    let i = Wrapping(operand1.unwrap_byte()) * Wrapping(operand2.unwrap_byte());
                    return Ok(InsertableNum::Byte(i.0))
                }          
            }, 
            TokenType::DIVIDE =>
            {

                // cannot divide by zero
                if operand2.unwrap() == 0
                {
                    return Err(Assembler::create_error("Cannot divide by zero", &operator, vec![]));
                }


                // return a u16
                if operand1.is_two_bytes()|| operand2.is_two_bytes()
                {
                    let i =  Wrapping(operand1.unwrap_twobyte()) / Wrapping(operand2.unwrap_twobyte());
                    return Ok(InsertableNum::TwoByte(i.0));
                }      
                else
                {
                    let i = Wrapping(operand1.unwrap_byte()) /  Wrapping(operand2.unwrap_byte());
                    return Ok(InsertableNum::Byte(i.0))
                }          
            }
            _ => {Ok(InsertableNum::Byte(0))}
        }
    }


