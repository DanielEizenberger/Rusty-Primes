using API.DTOs;
using Microsoft.AspNetCore.Mvc;

namespace API.Controllers;

[ApiController]
[Route("[controller]")]
public class JobController : ControllerBase
{
    private readonly ILogger<JobController> _logger;
    private readonly DBPlacerHolder _db;

    public JobController(ILogger<JobController> logger, DBPlacerHolder db)
    {
        _logger = logger;
        _db = db;
    }

    [HttpGet]
    public IActionResult GetNextJob()
    {
        long number = _db.PrimeQueue.FirstOrDefault(entry => entry.Value < DateTime.UtcNow).Key;
        if (number == 0)
        {
            _db.PrimeQueue.Add(_db.CurrentNumber, DateTime.UtcNow.Add(GetMaxCalculationTime(_db.CurrentNumber)));
            number = _db.CurrentNumber++;
        }
        else
        {
            _db.PrimeQueue[number] = DateTime.UtcNow.Add(GetMaxCalculationTime(number));
        }
        
        _logger.LogDebug($"Calling {nameof(GetNextJob)}; returning {number}");
        return Ok(number);
    }

    [HttpPost]
    public IActionResult PostCalculationResult([FromBody] PrimePostDto dto)
    {
        _logger.LogDebug($"Calling {nameof(PostCalculationResult)}; {nameof(dto.Number)}: {dto.Number}, {nameof(dto.IsPrime)}: {dto.IsPrime}");
        if (_db.PrimeQueue.Remove(dto.Number))
        {
            if (dto.IsPrime)
            {
                _logger.LogInformation($"Found a prime number: {dto.Number}");
                _db.Primes.Add(dto.Number);
            }
            return Created();
        }

        return BadRequest("Number currently not queued for prime checking.");
    }

    private TimeSpan GetMaxCalculationTime(long number)
    {
        TimeSpan networkTolerance = TimeSpan.FromSeconds(10);
        
        if (number <= 0) return TimeSpan.Zero;
        if (number < Int16.MaxValue) return networkTolerance;

        long x = number;
        long y = (x + 1) / 2;

        while (y < x)
        {
            x = y;
            y = (x + number / x) / 2;
        }

        return TimeSpan.FromMilliseconds(x).Add(networkTolerance);
    }
}
