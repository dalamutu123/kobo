## Question 1 
                                      Answer
The decision that a '.' begins a fractional part is made in number() on line 156, using the condition:
self.peek() == '.' && self.peek_next().is_ascii_digit()

This means the scanner only treats '.' as part of a number when there is a digit immediately after it. The scanner uses peek() to see the '.' and peek_next() to check the character after it, before consuming the '.'.

For the input 5., the scanner first consumes 5 as a number. It then checks the '.'. Although peek() sees '.', peek_next() does not see a digit because the number ends there. Therefore, the condition is false, so the scanner does not consume the .. The number() function finishes by producing NUMBER "5". The '.' is then processed separately by the next scan_token() call and reaches the default _ case, producing “Character is not part of any token.” This is required by Section 1.4 because 5. is not a valid Kobo number. 


## Question 2
                                        Answer
self.line tracks the current line as the scanner moves through the source. It is incremented in scan_token() on line 112, where a newline is handled with '\n' => self.line += 1, and also in string() on line 137, where self.line += 1 is executed when a newline occurs inside a string. These updates ensure that tokens are assigned to the correct lines while scanning.

However, self.line can continue increasing because of trailing blank lines after the final token. Therefore, in run(), the EOF line is calculated on line 35 using the last token in self.tokens, rather than using self.line directly. For example, if the last real token is ; on line 3 but the file ends with two blank lines, EOF should still be reported on line 3. Using self.tokens.last() gives the line of the last actual token scanned, while self.line may have moved forward because of whitespace. For an empty file, the fallback line is 1.

## Question 3

One test in tests/phase-1/ that I failed was the eof_line test. In run(), I initially used self.line directly on line 45 when creating the EOF token. I had misunderstood that self.line would still represent the line of the last real token after scanning finished. However, this is not always true. If the source file contains blank lines or trailing comments after the final token, each newline increments self.line, causing it to represent the physical last line instead of the line containing the last actual token. I fixed this by changing the EOF line calculation to use self.tokens.last().unwrap().line on line 38, with a fallback to 1 when there are no tokens. This ensures EOF is associated with the last real token rather than trailing whitespace. Unfortunately, I corrected this before committing the function, so there is no commit containing the incorrect version.