using Microsoft.AspNetCore.Mvc;

namespace API.Controllers;

[ApiController]
[Route("[controller]")]
public class PrimesController : ControllerBase
{
    private readonly ILogger<PrimesController> _logger;
    private readonly DBPlacerHolder _db;

    public PrimesController(ILogger<PrimesController> logger, DBPlacerHolder db)
    {
        _logger = logger;
        _db = db;
    }

    [HttpGet("count")]
    public IActionResult GetCount()
    {
        return Ok(_db.Primes.Count);
    }

    [HttpGet("latest")]
    public IActionResult GetLatest()
    {
        if (_db.Primes.Count == 0) return NoContent();
        return Ok(_db.Primes.Last());
    }

    [HttpGet("random")]
    public IActionResult GetRandom()
    {
        if (_db.Primes.Count == 0) return NoContent();
        
        Random rnd = new Random();
        return Ok(_db.Primes[rnd.Next(0, _db.Primes.Count)]);
    }
}
