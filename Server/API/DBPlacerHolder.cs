namespace API;

public class DBPlacerHolder
{
    public Dictionary<long, DateTime> PrimeQueue { get; } = new();
    public List<long> Primes { get; } = new();
    public long CurrentNumber { get; set; } = int.MaxValue;
}
