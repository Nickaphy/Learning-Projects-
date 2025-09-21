namespace BMI
{
    internal class BMICalc
    {
        static void Main(string[] args)
        {
            Console.Clear();
            Console.Write("Din vægt i KG: ");
            double bruger_vægt = Convert.ToDouble(Console.ReadLine());
            Console.Write("Din højde i meter: ");
            double bmi = BmiCalculator(bruger_vægt, bruger_højde); //Giver BmiCalculator parametrene og kalder funktionen
            Console.WriteLine($"Dit BMI er: {bmi:F2}"); //Interpoleret string (variabler kan indsættes, F2 er fixed decimal (2 decimaler)
        }

        public static double BmiCalculator(double vægt, double højde) //Funktionen modtager de 2 parametre
        {
            Console.
            return vægt / (højde * højde);
        }
    }
};
